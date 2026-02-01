use crate::domain::entity::FileEntity;

pub trait FileRepository {
    fn track(&self, file: FileEntity);
    fn remove(&self, id_path: String);
    fn list(&self) -> Vec<FileEntity>;
}
