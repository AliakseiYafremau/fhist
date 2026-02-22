use std::fs::OpenOptions;
use std::os::unix::io::AsRawFd;
use std::path::PathBuf;

use crate::AppError;
use crate::AppResult;
use crate::data_management::get_dir;

#[cfg(unix)]
pub fn daemonize() -> AppResult<()> {
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
pub fn daemonize() -> AppResult<()> {
    Err(AppError::Daemon(
        "daemon mode is only supported on unix targets".to_string(),
    ))
}

pub fn try_acquire_daemon_lock() -> AppResult<Option<std::fs::File>> {
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

pub fn start_background_watcher(
    db_path: PathBuf,
    spawn_watcher: impl Fn(PathBuf) -> AppResult<std::thread::JoinHandle<()>>,
) -> AppResult<()> {
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
            let handle = match spawn_watcher(db_path) {
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
