use crate::domain::entity::SnapshotEntity;

pub struct LogDTO {
    pub file_id: String,
    pub file_path: String,
    pub snapshots: Vec<SnapshotEntity>,
}

pub struct DiffDTO {
    pub file_id: String,
    pub file_path: String,
    pub from: SnapshotEntity,
    pub to: SnapshotEntity,
}

pub struct RollbackDTO {
    pub file_id: String,
    pub file_path: String,
    pub snapshot: SnapshotEntity,
}
