use std::fs::File;

use chrono::Utc;
use rusqlite::Connection;

use crate::domain::entity::{FileEntity, SnapshotEntity};
use crate::domain::repository::FileRepository;

pub struct SQLFileRepository {
    pub connection: Connection,
}
