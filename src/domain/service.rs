use chrono::Utc;
use uuid::Uuid;

use crate::domain::dto::{DiffDTO, LogDTO, RollbackDTO};
use crate::domain::entity::FileEntity;
use crate::domain::factory::{build_file, build_snapshot};
use crate::domain::repository::{FileRepository, SnapshotRepository};
use crate::domain::uuid_util::{uuid_generate, uuid_to_str};

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

pub fn get_info(
    file_id_path: &str,
    file_repository: &impl FileRepository,
    snap_repository: &impl SnapshotRepository,
) -> LogDTO {
    let file_entity = file_repository.get_by_id_or_path(file_id_path);
    let file_snapshots = snap_repository.get_by_id_or_path(&file_entity.id);

    LogDTO {
        file_id: file_entity.id,
        file_path: file_entity.path,
        snapshots: file_snapshots,
    }
}

pub fn get_diff(
    file_id_path: &str,
    from_id: Option<&str>,
    to_id: Option<&str>,
    file_repository: &impl FileRepository,
    snap_repository: &impl SnapshotRepository,
) -> Result<DiffDTO, String> {
    let file_entity = file_repository.get_by_id_or_path(file_id_path);
    let file_snapshots = snap_repository.get_by_id_or_path(&file_entity.id);
    if file_snapshots.len() < 2 {
        return Err("not enough snapshots to diff".to_string());
    }

    let (from, to) = match (from_id, to_id) {
        (Some(from), Some(to)) => {
            let from_snapshot = file_snapshots
                .iter()
                .find(|snapshot| snapshot.id == from)
                .cloned()
                .ok_or_else(|| format!("snapshot \"{from}\" not found"))?;
            let to_snapshot = file_snapshots
                .iter()
                .find(|snapshot| snapshot.id == to)
                .cloned()
                .ok_or_else(|| format!("snapshot \"{to}\" not found"))?;
            (from_snapshot, to_snapshot)
        }
        (Some(from), None) => {
            let from_snapshot = file_snapshots
                .iter()
                .find(|snapshot| snapshot.id == from)
                .cloned()
                .ok_or_else(|| format!("snapshot \"{from}\" not found"))?;
            let to_snapshot = file_snapshots
                .last()
                .cloned()
                .ok_or_else(|| "not enough snapshots to diff".to_string())?;
            (from_snapshot, to_snapshot)
        }
        (None, Some(to)) => {
            let index = file_snapshots
                .iter()
                .position(|snapshot| snapshot.id == to)
                .ok_or_else(|| format!("snapshot \"{to}\" not found"))?;
            if index == 0 {
                return Err("no snapshot available before the target snapshot".to_string());
            }
            let from_snapshot = file_snapshots[index - 1].clone();
            let to_snapshot = file_snapshots[index].clone();
            (from_snapshot, to_snapshot)
        }
        (None, None) => {
            let to_snapshot = file_snapshots
                .last()
                .cloned()
                .ok_or_else(|| "not enough snapshots to diff".to_string())?;
            let from_snapshot = file_snapshots
                .get(file_snapshots.len() - 2)
                .cloned()
                .ok_or_else(|| "not enough snapshots to diff".to_string())?;
            (from_snapshot, to_snapshot)
        }
    };

    Ok(DiffDTO {
        file_id: file_entity.id,
        file_path: file_entity.path,
        from,
        to,
    })
}

pub fn get_rollback(
    file_id_path: &str,
    snapshot_id: Option<&str>,
    file_repository: &impl FileRepository,
    snap_repository: &impl SnapshotRepository,
) -> Result<RollbackDTO, String> {
    let file_entity = file_repository.get_by_id_or_path(file_id_path);
    let file_snapshots = snap_repository.get_by_id_or_path(&file_entity.id);
    if file_snapshots.is_empty() {
        return Err("no snapshots available to roll back".to_string());
    }

    let snapshot = match snapshot_id {
        Some(id) => file_snapshots
            .iter()
            .find(|item| item.id == id)
            .cloned()
            .ok_or_else(|| format!("snapshot \"{id}\" not found"))?,
        None => file_snapshots
            .last()
            .cloned()
            .ok_or_else(|| "no snapshots available to roll back".to_string())?,
    };

    Ok(RollbackDTO {
        file_id: file_entity.id,
        file_path: file_entity.path,
        snapshot,
    })
}

pub fn add_snapshot(file_id_path: &str, content: &str, snap_repository: &impl SnapshotRepository) {
    let snapshot = build_snapshot(&uuid_generate(), Utc::now(), content);
    snap_repository.add(file_id_path, snapshot);
}

pub fn remove_snapshot(snapshot_id: &str, snap_repository: &impl SnapshotRepository) {
    snap_repository.delete_by_snapshot_id(snapshot_id);
}
