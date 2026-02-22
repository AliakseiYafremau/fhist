mod adapters;
mod cli;
mod data_management;
mod domain;
mod output;

use clap::Parser;
use notify::{EventKind, RecursiveMode, Watcher, recommended_watcher};
use rusqlite::Connection;
use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::os::unix::io::AsRawFd;
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use std::thread;

use crate::adapters::sql::{SQLFileRepository, SQLSnapshotRepository};
use crate::cli::{Args, Commands};
use crate::data_management::{ensure_dir, get_dir};
use crate::domain::service::{add_snapshot, get_info, list, start_track_file, stop_to_track_file};
use crate::output::{output_file_info, output_snapshot_info};

#[allow(dead_code)]
#[derive(Debug)]
enum AppError {
    Io(std::io::Error),
    Db(rusqlite::Error),
    No(notify::Error),
    Daemon(String),
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::Io(e)
    }
}

impl From<rusqlite::Error> for AppError {
    fn from(e: rusqlite::Error) -> Self {
        AppError::Db(e)
    }
}

impl From<notify::Error> for AppError {
    fn from(e: notify::Error) -> Self {
        AppError::No(e)
    }
}

type AppResult<T> = Result<T, AppError>;

#[cfg(unix)]
fn daemonize() -> AppResult<()> {
    unsafe {
        let pid = libc::fork();
        if pid < 0 {
            return Err(AppError::Daemon("fork failed".to_string()));
        }
        if pid > 0 {
            std::process::exit(0);
        }

        if libc::setsid() < 0 {
            return Err(AppError::Daemon("setsid failed".to_string()));
        }

        let pid = libc::fork();
        if pid < 0 {
            return Err(AppError::Daemon("second fork failed".to_string()));
        }
        if pid > 0 {
            std::process::exit(0);
        }

        libc::umask(0);
        let root = std::ffi::CString::new("/").expect("failed to build root cstring");
        if libc::chdir(root.as_ptr()) < 0 {
            return Err(AppError::Daemon("chdir failed".to_string()));
        }

        let devnull = std::ffi::CString::new("/dev/null").expect("failed to build devnull");
        let fd = libc::open(devnull.as_ptr(), libc::O_RDWR);
        if fd >= 0 {
            libc::dup2(fd, libc::STDIN_FILENO);
            libc::dup2(fd, libc::STDOUT_FILENO);
            libc::dup2(fd, libc::STDERR_FILENO);
            if fd > libc::STDERR_FILENO {
                libc::close(fd);
            }
        }
    }

    Ok(())
}

#[cfg(not(unix))]
fn daemonize() -> AppResult<()> {
    Err(AppError::Daemon(
        "daemon mode is only supported on unix targets".to_string(),
    ))
}

fn spawn_snapshot_watcher(
    db_path: PathBuf,
    file_paths: Vec<String>,
) -> AppResult<thread::JoinHandle<()>> {
    let handle = thread::spawn(move || {
        let (tx, rx) = channel();
        let mut watcher = match recommended_watcher(tx) {
            Ok(watcher) => watcher,
            Err(err) => {
                eprintln!("failed to create watcher: {err}");
                return;
            }
        };

        for path in file_paths {
            if let Err(err) = watcher.watch(Path::new(&path), RecursiveMode::NonRecursive) {
                eprintln!("failed to watch file {path}: {err}");
            }
        }

        let snapshot_connection = match Connection::open(&db_path) {
            Ok(connection) => connection,
            Err(err) => {
                eprintln!("failed to open snapshot db: {err}");
                return;
            }
        };
        let sql_snapshot_repository = SQLSnapshotRepository {
            connection: snapshot_connection,
        };

        let mut last_content: HashMap<PathBuf, String> = HashMap::new();

        for event in rx {
            let event = match event {
                Ok(event) => event,
                Err(err) => {
                    eprintln!("watcher error: {err}");
                    continue;
                }
            };

            match event.kind {
                EventKind::Modify(_) | EventKind::Create(_) => {}
                _ => continue,
            }

            for path in event.paths {
                let content = match fs::read_to_string(&path) {
                    Ok(content) => content,
                    Err(err) => {
                        eprintln!("failed to read file {}: {err}", path.display());
                        continue;
                    }
                };
                if last_content.get(&path).map(|prev| prev == &content) == Some(true) {
                    continue;
                }
                last_content.insert(path.clone(), content.clone());
                let file_id_path = path.to_string_lossy();
                add_snapshot(&file_id_path, &content, &sql_snapshot_repository);
            }
        }
    });

    Ok(handle)
}

fn start_background_watcher(db_path: PathBuf, file_paths: Vec<String>) -> AppResult<()> {
    let lock = match try_acquire_daemon_lock()? {
        Some(lock) => lock,
        None => return Ok(()),
    };

    unsafe {
        let pid = libc::fork();
        if pid < 0 {
            return Err(AppError::Daemon("fork failed".to_string()));
        }
        if pid == 0 {
            if let Err(err) = daemonize() {
                eprintln!("daemonize failed: {err:?}");
                std::process::exit(1);
            }
            std::mem::forget(lock);
            let handle = match spawn_snapshot_watcher(db_path, file_paths) {
                Ok(handle) => handle,
                Err(err) => {
                    eprintln!("failed to start watcher: {err:?}");
                    std::process::exit(1);
                }
            };
            let _ = handle.join();
            std::process::exit(0);
        }
    }

    Ok(())
}

fn try_acquire_daemon_lock() -> AppResult<Option<std::fs::File>> {
    let data_dir = get_dir()?;
    let lock_path = data_dir.join("fhist.daemon.lock");
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(lock_path)?;

    let rc = unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) };
    if rc != 0 {
        return Ok(None);
    }

    Ok(Some(file))
}

fn main() -> AppResult<()> {
    let args = Args::parse();

    ensure_dir()?;
    let data_dir = get_dir()?;
    let db_path = data_dir.join("fhist.sqlite");

    let file_connection = Connection::open(&db_path)?;
    let snapshot_connection = Connection::open(&db_path)?;
    let sql_file_repository = SQLFileRepository {
        connection: file_connection,
    };
    let sql_snapshot_repository = SQLSnapshotRepository {
        connection: snapshot_connection,
    };

    if !matches!(args.command, Commands::Daemon) {
        let tracked_paths = list(&sql_file_repository)
            .into_iter()
            .map(|file| file.path)
            .collect();
        start_background_watcher(db_path.clone(), tracked_paths)?;
    }

    match args.command {
        Commands::Add { target } => start_track_file(&target, &sql_file_repository),
        Commands::Remove { target } => {
            stop_to_track_file(&target, &sql_file_repository, &sql_snapshot_repository)
        }
        Commands::List => {
            let files = list(&sql_file_repository);
            for file in files {
                output_file_info(&file.id, &file.path);
            }
        }
        Commands::Log { target } => {
            let log = get_info(&target, &sql_file_repository, &sql_snapshot_repository);

            output_file_info(&log.file_id, &log.file_path);
            for snapshot in log.snapshots {
                output_snapshot_info(&snapshot.id, snapshot.date, &snapshot.content);
            }
        }
        Commands::Watch => {
            let tracked_paths = list(&sql_file_repository)
                .into_iter()
                .map(|file| file.path)
                .collect();
            let handle = spawn_snapshot_watcher(db_path.clone(), tracked_paths)?;
            let _ = handle.join();
        }
        Commands::Daemon => {
            daemonize()?;
            let tracked_paths = list(&sql_file_repository)
                .into_iter()
                .map(|file| file.path)
                .collect();
            let handle = spawn_snapshot_watcher(db_path.clone(), tracked_paths)?;
            let _ = handle.join();
        }
    }

    Ok(())
}
