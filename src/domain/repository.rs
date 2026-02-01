use crate::domain::file::FileEntity;

pub trait FileRepository {
    fn track(&self, file: FileEntity);
    fn remove(&self, id_path: String);
    fn list(&self) -> Vec<FileEntity>;
}
