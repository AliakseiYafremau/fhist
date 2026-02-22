use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use std::thread;
use std::time::{Duration, Instant};

use notify::{EventKind, RecursiveMode, Watcher, recommended_watcher};
use rusqlite::Connection;

use crate::adapters::sql::{SQLFileRepository, SQLSnapshotRepository};
use crate::domain::repository::FileRepository;
use crate::domain::service::add_snapshot;

pub fn spawn_snapshot_watcher(db_path: PathBuf) -> crate::AppResult<thread::JoinHandle<()>> {
    let handle = thread::spawn(move || {
        let (tx, rx) = channel();
        let mut watcher = match recommended_watcher(tx) {
            Ok(watcher) => watcher,
            Err(err) => {
                eprintln!("failed to create watcher: {err}");
                return;
            }
        };

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
        let file_connection = match Connection::open(&db_path) {
            Ok(connection) => connection,
            Err(err) => {
                eprintln!("failed to open file db: {err}");
                return;
            }
        };
        let sql_file_repository = SQLFileRepository {
            connection: file_connection,
        };

        let mut last_content: HashMap<PathBuf, String> = HashMap::new();
        let mut watched: HashSet<PathBuf> = HashSet::new();
        let mut last_refresh = Instant::now();

        loop {
            if last_refresh.elapsed() >= Duration::from_secs(2) {
                let files = sql_file_repository.list();
                for file in files {
                    let path = PathBuf::from(file.path);
                    if watched.insert(path.clone()) {
                        if let Err(err) =
                            watcher.watch(Path::new(&path), RecursiveMode::NonRecursive)
                        {
                            eprintln!("failed to watch file {}: {err}", path.display());
                            watched.remove(&path);
                        }
                    }
                }
                last_refresh = Instant::now();
            }

            let event = match rx.recv_timeout(Duration::from_millis(200)) {
                Ok(event) => event,
                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => continue,
                Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => break,
            };

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
