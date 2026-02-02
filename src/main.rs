mod adapters;
mod cli;
mod data_management;
mod domain;

use clap::Parser;
use std::io::Error;

use crate::domain::dto::FileDTO;
use crate::domain::service::{list, stop_to_track_file, start_track_file};

use crate::adapters::repository::LocalRepositoty;

use crate::cli::Args;
use crate::data_management::ensure_dir;

fn main() -> Result<(), Error> {
    ensure_dir()?;

    let args = Args::parse();

    let local_repository = LocalRepositoty;

    let first_file_dto = FileDTO {
        id: "1".to_string(),
        path: "first/path".to_string(),
    };
    let second_file_dto = FileDTO {
        id: "2".to_string(),
        path: "second/path".to_string(),
    };

    start_track_file(first_file_dto, &local_repository);
    start_track_file(second_file_dto, &local_repository);

    stop_to_track_file("1 id".to_string(), &local_repository);
    stop_to_track_file("2 path".to_string(), &local_repository);

    list(&local_repository);
    list(&local_repository);

    Ok(())
}
