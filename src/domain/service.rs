use crate::domain::dto::{FileDTO, map_to_file_dto};
use crate::domain::factory::build_file;
use crate::domain::repository::FileRepository;


fn track_file(file_dto: FileDTO, repository: impl FileRepository) {
    let file_to_track = build_file(file_dto.id, file_dto.path);
    repository.track(file_to_track);
}

fn remove(id_path: String, repository: impl FileRepository) {
    repository.remove(id_path);
}

fn list(repository: impl FileRepository) -> Vec<FileDTO> {
    let file_entities = repository.list();

    let mut file_dtos = Vec::with_capacity(file_entities.len());
    for entity in file_entities {
        file_dtos.push(map_to_file_dto(entity));
    }

    file_dtos
}