mod adapters;
mod cli;
mod data_management;
mod domain;
mod output;

use clap::Parser;

use rusqlite::Connection;
// use std::path::Path;
// use std::sync::mpsc::channel;
// use notify::{Event, RecursiveMode, Watcher, recommended_watcher};

use crate::adapters::sql::{SQLFileRepository, SQLSnapshotRepository};
use crate::cli::{Args, Commands};
use crate::data_management::{ensure_dir, get_dir};
use crate::domain::service::{get_info, list, start_track_file, stop_to_track_file};
use crate::output::{output_file_info, output_snapshot_info};

#[allow(dead_code)]
#[derive(Debug)]
enum AppError {
    Io(std::io::Error),
    Db(rusqlite::Error),
    No(notify::Error),
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

fn main() -> AppResult<()> {
    // let (tx, rx) = channel::<notify::Result<Event>>();

    // let mut watcher = recommended_watcher(tx)?;
    // watcher.watch(Path::new(s), recursive_mode);

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
    }

    Ok(())
}
