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
use crate::domain::service::{
    get_diff, get_info, get_rollback, list, start_track_file, stop_to_track_file,
};
use crate::notify_watcher::spawn_snapshot_watcher;
use crate::output::{output_diff, output_file_info, output_rollback, output_snapshot_info};

#[derive(Debug)]
pub enum AppError {
    Io(std::io::Error),
    Db(rusqlite::Error),
    No(notify::Error),
    Daemon(String),
    Usage(String),
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
        Commands::Log { target, verbose } => {
            let resolved = resolve_path(&target);
            let log = get_info(&resolved, &sql_file_repository, &sql_snapshot_repository);

            output_file_info(&log.file_id, &log.file_path);
            for snapshot in log.snapshots {
                output_snapshot_info(&snapshot.id, snapshot.date, &snapshot.content, verbose);
            }
        }
        Commands::Diff { target, from, to } => {
            let resolved = resolve_path(&target);
            let diff = get_diff(
                &resolved,
                from.as_deref(),
                to.as_deref(),
                &sql_file_repository,
                &sql_snapshot_repository,
            )
            .map_err(AppError::Usage)?;
            output_diff(
                &diff.file_id,
                &diff.file_path,
                &diff.from.id,
                diff.from.date,
                &diff.from.content,
                &diff.to.id,
                diff.to.date,
                &diff.to.content,
            );
        }
        Commands::Rollback { target, snapshot } => {
            let resolved = resolve_path(&target);
            let rollback = get_rollback(
                &resolved,
                snapshot.as_deref(),
                &sql_file_repository,
                &sql_snapshot_repository,
            )
            .map_err(AppError::Usage)?;
            std::fs::write(&rollback.file_path, rollback.snapshot.content).map_err(AppError::Io)?;
            output_rollback(&rollback.file_id, &rollback.file_path, &rollback.snapshot.id);
        }
    }

    Ok(())
}
