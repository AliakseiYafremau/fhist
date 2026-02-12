use chrono::{DateTime, Utc};

use crate::domain::entity::{FileEntity, SnapshotEntity};

pub fn build_file(id: &str, path: &str) -> FileEntity {
    FileEntity {
        id: id.to_string(),
        path: path.to_string(),
    }
}

pub fn build_snapshot(id: &str, date: DateTime<Utc>, content: &str) -> SnapshotEntity {
    SnapshotEntity {
        id: id.to_string(),
        date,
        content: content.to_string(),
    }
}
