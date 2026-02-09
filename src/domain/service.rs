use uuid::Uuid;

use crate::domain::uuid_util::uuid_to_str;
use crate::domain::dto::{FileDTO, map_to_file_dto};
use crate::domain::factory::build_file;
use crate::domain::repository::{FileRepository, SnapshotRepository};

pub fn start_track_file(file_path: &str, repository: &impl FileRepository) {
    let file_id = uuid_to_str(Uuid::new_v4());
    let file_to_track = build_file(&file_id, file_path);
    repository.track(file_to_track);
}

pub fn stop_to_track_file(
    id_path: &str,
    file_repository: &impl FileRepository,
    snap_repository: &impl SnapshotRepository,
) {
    snap_repository.delete_by_file_id_path(id_path);
    file_repository.remove(id_path);
}

pub fn list(repository: &impl FileRepository) -> Vec<FileDTO> {
    let file_entities = repository.list();

    let mut file_dtos = Vec::with_capacity(file_entities.len());
    for entity in file_entities {
        file_dtos.push(map_to_file_dto(entity));
    }

    file_dtos
}

pub fn add_snapshot(file_id_path: &str, snapshot: String, repository: &impl SnapshotRepository) {
    repository.add(file_id_path, snapshot);
}
