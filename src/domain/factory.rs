use crate::domain::entity::FileEntity;

pub fn build_file(id: String, path: String) -> FileEntity {
    FileEntity { id: id, path: path }
}
