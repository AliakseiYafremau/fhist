use chrono::Utc;
use uuid::Uuid;

use crate::domain::dto::LogDTO;
use crate::domain::entity::FileEntity;
use crate::domain::uuid_util::{uuid_generate, uuid_to_str};
use crate::domain::factory::{build_file, build_snapshot};
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
    snap_repository.delete_all_by_file_id_path(id_path);
    file_repository.remove(id_path);
}

pub fn list(repository: &impl FileRepository) -> Vec<FileEntity> {
    let file_entities = repository.list();

    file_entities
}

pub fn get_info(file_id_path: &str, file_repository: &impl FileRepository, snap_repository: &impl SnapshotRepository) -> LogDTO {
    let file_entity = file_repository.get_by_id_or_path(file_id_path);
    let file_snapshots = snap_repository.get_by_id_or_path(&file_entity.id);

    LogDTO { file_id: file_entity.id, file_path: file_entity.path, snapshots: file_snapshots }
}

pub fn add_snapshot(
    file_id_path: &str,
    content: &str,
    snap_repository: &impl SnapshotRepository,
) {
    let snapshot = build_snapshot(&uuid_generate(), Utc::now(), content);
    snap_repository.add(file_id_path, snapshot);
}

pub fn remove_snapshot(
    snapshot_id: &str,
    snap_repository: &impl SnapshotRepository,
) {
    snap_repository.delete_by_snapshot_id(snapshot_id);
}