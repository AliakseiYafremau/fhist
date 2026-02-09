use crate::domain::entity::FileEntity;

pub fn build_file(id: &str, path: &str) -> FileEntity {
    FileEntity {
        id: id.to_string(),
        path: path.to_string(),
    }
}
