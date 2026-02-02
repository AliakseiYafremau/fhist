use std::fs::File;

use chrono::Utc;

use crate::domain::entity::{FileEntity, SnapshotEntity};
use crate::domain::repository::{FileRepository, SnapshotRepository};

pub struct LocalFileRepositoty;
pub struct LocalSnapshotRepository;

impl FileRepository for LocalFileRepositoty {
    fn track(&self, file: FileEntity) {
        println!("File with path \"{}\" was tracked", file.path);
    }
    fn remove(&self, id_path: &str) {
        println!("File with path(or id)\"{}\" was removed", id_path);
    }
    fn update(&self, file: FileEntity) {
        println!("File with id\"{}\" was updated", file.id);
    }
    fn get_by_id_or_path(&self, id_path: &str) -> FileEntity{
        FileEntity {
            id: "id example".to_string(),
            path: "path example".to_string(),
        }
    }
    fn list(&self) -> Vec<FileEntity> {
        vec![FileEntity {
            id: "id example".to_string(),
            path: "path example".to_string(),
        }]
    }
}

impl SnapshotRepository for LocalSnapshotRepository {
    fn delete_by_file_id_path(&self, file_id_path: &str) {
        println!("Snapshots for file \"{}\" were deleted", file_id_path);
    }

    fn add(&self, file_id_path: &str, snapshot: String) {
        println!(
            "Snapshot \"{}\" was added for file \"{}\"",
            snapshot, file_id_path
        );
    }

    fn get_by_id_or_path(&self, file_id_path: &str) -> Vec<SnapshotEntity> {
        println!("Snapshots for file \"{}\" were requested", file_id_path);
        vec![SnapshotEntity {
            id: "id example".to_string(),
            date: Utc::now(),
        }]
    }
}
