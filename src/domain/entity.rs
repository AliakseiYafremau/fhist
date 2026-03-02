use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct FileEntity {
    pub id: String,
    pub path: String,
}

#[derive(Clone)]
pub struct SnapshotEntity {
    pub id: String,
    pub date: DateTime<Utc>,
    pub content: String,
}
