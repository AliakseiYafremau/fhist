mod domain;
mod adapters;
mod cli;

use clap::Parser;
use crate::cli::Args;
use crate::domain::dto::FileDTO;
use crate::domain::service::{track_file, remove, list};
use crate::adapters::repository::LocalRepositoty;

fn main() {
    let args = Args::parse();


    let local_repository = LocalRepositoty;

    let first_file_dto = FileDTO{id: "1".to_string(), path: "first/path".to_string()};
    let second_file_dto = FileDTO{id: "2".to_string(), path: "second/path".to_string()};

    track_file(first_file_dto, &local_repository);
    track_file(second_file_dto, &local_repository);

    remove("1 id".to_string(), &local_repository);
    remove("2 path".to_string(), &local_repository);

    list(&local_repository);
    list(&local_repository);
}