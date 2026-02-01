use crate::domain::repository::FileRepository;
use crate::domain::entity::FileEntity;

pub struct LocalRepositoty;

impl FileRepository for LocalRepositoty {
    fn track(&self, file: FileEntity) {
        println!("File with path \"{}\" was tracked", file.path);
    }
    fn remove(&self, id_path: String) {
        println!("File with path(or id)\"{}\" was removed", id_path);
    }
    fn list(&self) -> Vec<FileEntity> {
        vec![FileEntity {
            id: "id example".to_string(),
            path: "path example".to_string(),
        }]
    }
}