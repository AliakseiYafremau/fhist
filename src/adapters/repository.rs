use std::fs::File;

use crate::domain::entity::FileEntity;
use crate::domain::repository::FileRepository;

pub struct LocalRepositoty;

impl FileRepository for LocalRepositoty {
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
