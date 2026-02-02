use crate::domain::entity::FileEntity;

pub struct FileDTO {
    pub id: String,
    pub path: String,
}

pub fn map_to_file_dto(file_entity: FileEntity) -> FileDTO {
    FileDTO {
        id: file_entity.id,
        path: file_entity.path,
    }
}
