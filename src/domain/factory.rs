use crate::domain::file::FileEntity;

pub fn build_file(id: String, path: String) -> FileEntity {
    FileEntity { id: id, path: path }
}
