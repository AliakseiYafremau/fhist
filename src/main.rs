mod adapters;
mod cli;
mod daemon;
mod data_management;
mod domain;
mod notify_watcher;
mod output;

use clap::Parser;
use rusqlite::Connection;
use std::path::Path;

use crate::adapters::sql::{SQLFileRepository, SQLSnapshotRepository};
use crate::cli::{Args, Commands};
use crate::daemon::start_background_watcher;
use crate::data_management::{ensure_dir, get_dir};
use crate::domain::service::{get_info, list, start_track_file, stop_to_track_file};
use crate::notify_watcher::spawn_snapshot_watcher;
use crate::output::{output_file_info, output_snapshot_info};

#[derive(Debug)]
pub enum AppError {
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

pub type AppResult<T> = Result<T, AppError>;

fn resolve_path(input: &str) -> String {
    let path = Path::new(input);
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        match std::env::current_dir() {
            Ok(dir) => dir.join(path),
            Err(_) => path.to_path_buf(),
        }
    };

    std::fs::canonicalize(&absolute)
        .unwrap_or(absolute)
        .to_string_lossy()
        .into_owned()
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

    start_background_watcher(db_path.clone(), spawn_snapshot_watcher)?;

    match args.command {
        Commands::Add { target } => {
            let resolved = resolve_path(&target);
            start_track_file(&resolved, &sql_file_repository)
        }
        Commands::Remove { target } => {
            let resolved = resolve_path(&target);
            stop_to_track_file(&resolved, &sql_file_repository, &sql_snapshot_repository)
        }
        Commands::List => {
            let files = list(&sql_file_repository);
            for file in files {
                output_file_info(&file.id, &file.path);
            }
        }
        Commands::Log { target } => {
            let resolved = resolve_path(&target);
            let log = get_info(&resolved, &sql_file_repository, &sql_snapshot_repository);

            output_file_info(&log.file_id, &log.file_path);
            for snapshot in log.snapshots {
                output_snapshot_info(&snapshot.id, snapshot.date, &snapshot.content);
            }
        }
    }

    Ok(())
}
