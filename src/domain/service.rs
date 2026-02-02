use crate::domain::dto::{FileDTO, map_to_file_dto};
use crate::domain::factory::build_file;
use crate::domain::repository::{FileRepository, SnapshotRepository};

pub fn start_track_file(file_dto: FileDTO, repository: &impl FileRepository) {
    let file_to_track = build_file(file_dto.id, file_dto.path);
    repository.track(file_to_track);
}

pub fn stop_to_track_file(id_path: String, file_repository: &impl FileRepository, snap_repository: &impl SnapshotRepository) {
    snap_repository.delete_by_file_id_path(&id_path);
    file_repository.remove(&id_path);
}

pub fn list(repository: &impl FileRepository) -> Vec<FileDTO> {
    let file_entities = repository.list();

    let mut file_dtos = Vec::with_capacity(file_entities.len());
    for entity in file_entities {
        file_dtos.push(map_to_file_dto(entity));
    }

    file_dtos
}

// pub fn add_snapshot(id_path: String, ) {

// }
