mod adapters;
mod cli;
mod data_management;
mod domain;

use clap::Parser;

use crate::adapters::local::{self, LocalFileRepositoty, LocalSnapshotRepository};
use crate::cli::{Args, Commands};
use crate::domain::service::{list, start_track_file, stop_to_track_file};

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
    // // ensure_dir()?;
    // let data_dir = get_dir()?;

    let args = Args::parse();

    // let connection = Connection::open(&data_dir)?;
    // let _sql_file_repository = SQLFileRepository { connection };

    let local_file_repository = LocalFileRepositoty;
    let local_snapshot_repository = LocalSnapshotRepository;

    match args.command {
        Commands::Add { target } => start_track_file(&target, &local_file_repository),
        Commands::Remove { target } => stop_to_track_file(&target, &local_file_repository, &local_snapshot_repository),
        Commands::List => {
            let files = list(&local_file_repository);
            for file in files {
                println!("File: id - {}, path - {}", file.id, file.path);
            }
        }
        Commands::Log { target } => println!("Not implemented"),
    }

    Ok(())
}
