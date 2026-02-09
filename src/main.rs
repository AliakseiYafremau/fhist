mod adapters;
mod cli;
mod data_management;
mod domain;

use clap::Parser;
use rusqlite::Connection;

use crate::adapters::local::LocalSnapshotRepository;
use crate::adapters::sql::SQLFileRepository;
use crate::cli::Args;
use crate::data_management::{ensure_dir, get_dir};

#[derive(Debug)]
enum AppError {
    Io(std::io::Error),
    Db(rusqlite::Error),
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

type AppResult<T> = Result<T, AppError>;

fn main() -> AppResult<()> {
    ensure_dir()?;
    let data_dir = get_dir()?;

    let _args = Args::parse();

    let connection = Connection::open(&data_dir)?;

    let _local_file_repository = SQLFileRepository { connection };
    let _local_snapshot_repository = LocalSnapshotRepository;

    Ok(())
}
