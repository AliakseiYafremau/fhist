use chrono::{DateTime, Utc};

pub struct FileEntity {
    pub id: String,
    pub path: String,
}

pub struct SnapshotEntity {
    pub id: String,
    pub date: DateTime<Utc>,
}