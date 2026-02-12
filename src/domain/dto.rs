use crate::domain::entity::SnapshotEntity;

pub struct LogDTO {
    pub file_id: String,
    pub file_path: String,
    pub snapshots: Vec<SnapshotEntity>
}