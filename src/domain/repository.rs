use crate::domain::entity::{FileEntity, SnapshotEntity};

pub trait FileRepository {
    fn track(&self, file: FileEntity);
    fn remove(&self, id_path: &str);
    fn update(&self, file: FileEntity);
    fn get_by_id_or_path(&self, id_path: &str) -> FileEntity;
    fn list(&self) -> Vec<FileEntity>;
}

pub trait SnapshotRepository {
    fn delete_by_file_id_path(&self, file_id_path: &str);
    fn add(&self, file_id_path: &str, snapshot: String);
    fn get_by_id_or_path(&self, file_id_path: &str) -> Vec<SnapshotEntity>;
}
