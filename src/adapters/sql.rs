#![allow(dead_code)]

use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, OptionalExtension};

use crate::domain::entity::{FileEntity, SnapshotEntity};
use crate::domain::repository::{FileRepository, SnapshotRepository};

pub struct SQLFileRepository {
    pub connection: Connection,
}

pub struct SQLSnapshotRepository {
    pub connection: Connection,
}

fn ensure_schema(connection: &Connection) {
    connection
        .execute_batch(
            r#"
            PRAGMA foreign_keys = ON;
            CREATE TABLE IF NOT EXISTS files (
                id   TEXT PRIMARY KEY,
                path TEXT NOT NULL UNIQUE
            );
            CREATE TABLE IF NOT EXISTS snapshots (
                id      TEXT PRIMARY KEY,
                file_id TEXT NOT NULL,
                date    TEXT NOT NULL,
                content TEXT NOT NULL,
                FOREIGN KEY (file_id) REFERENCES files(id) ON DELETE CASCADE
            );
            "#,
        )
        .expect("failed to ensure schema");
}

fn parse_datetime(value: String) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(&value)
        .expect("failed to parse snapshot date")
        .with_timezone(&Utc)
}

fn get_file_id(connection: &Connection, id_path: &str) -> Option<String> {
    connection
        .query_row(
            "SELECT id FROM files WHERE id = ?1 OR path = ?1",
            params![id_path],
            |row| row.get(0),
        )
        .optional()
        .expect("failed to query file id")
}

impl FileRepository for SQLFileRepository {
    fn track(&self, file: FileEntity) {
        ensure_schema(&self.connection);
        self.connection
            .execute(
                "INSERT INTO files (id, path) VALUES (?1, ?2)",
                params![file.id, file.path],
            )
            .expect("failed to insert file");
    }

    fn remove(&self, id_path: &str) {
        ensure_schema(&self.connection);
        self.connection
            .execute(
                "DELETE FROM files WHERE id = ?1 OR path = ?1",
                params![id_path],
            )
            .expect("failed to delete file");
    }

    fn update(&self, file: FileEntity) {
        ensure_schema(&self.connection);
        self.connection
            .execute(
                "UPDATE files SET path = ?2 WHERE id = ?1",
                params![file.id, file.path],
            )
            .expect("failed to update file");
    }

    fn get_by_id_or_path(&self, id_path: &str) -> FileEntity {
        ensure_schema(&self.connection);
        self.connection
            .query_row(
                "SELECT id, path FROM files WHERE id = ?1 OR path = ?1",
                params![id_path],
                |row| {
                    Ok(FileEntity {
                        id: row.get(0)?,
                        path: row.get(1)?,
                    })
                },
            )
            .expect("file not found")
    }

    fn list(&self) -> Vec<FileEntity> {
        ensure_schema(&self.connection);
        let mut stmt = self
            .connection
            .prepare("SELECT id, path FROM files ORDER BY path")
            .expect("failed to prepare file list query");
        let rows = stmt
            .query_map([], |row| {
                Ok(FileEntity {
                    id: row.get(0)?,
                    path: row.get(1)?,
                })
            })
            .expect("failed to read files");
        rows.map(|row| row.expect("failed to read file row"))
            .collect()
    }
}

impl SnapshotRepository for SQLSnapshotRepository {
    fn delete_all_by_file_id_path(&self, file_id_path: &str) {
        ensure_schema(&self.connection);
        let Some(file_id) = get_file_id(&self.connection, file_id_path) else {
            return;
        };
        self.connection
            .execute(
                "DELETE FROM snapshots WHERE file_id = ?1",
                params![file_id],
            )
            .expect("failed to delete snapshots");
    }

    fn delete_by_snapshot_id(&self, snapshot_id: &str) {
        ensure_schema(&self.connection);
        self.connection
            .execute("DELETE FROM snapshots WHERE id = ?1", params![snapshot_id])
            .expect("failed to delete snapshot");
    }

    fn add(&self, file_id_path: &str, snapshot: SnapshotEntity) {
        ensure_schema(&self.connection);
        let Some(file_id) = get_file_id(&self.connection, file_id_path) else {
            return;
        };
        let snapshot_id = snapshot.id;
        let date = snapshot.date.to_rfc3339();
        self.connection
            .execute(
                "INSERT INTO snapshots (id, file_id, date, content) VALUES (?1, ?2, ?3, ?4)",
                params![snapshot_id, file_id, date, snapshot.content],
            )
            .expect("failed to insert snapshot");
    }

    fn get_by_id_or_path(&self, file_id_path: &str) -> Vec<SnapshotEntity> {
        ensure_schema(&self.connection);
        let Some(file_id) = get_file_id(&self.connection, file_id_path) else {
            return Vec::new();
        };
        let mut stmt = self
            .connection
            .prepare("SELECT id, date, content FROM snapshots WHERE file_id = ?1 ORDER BY date")
            .expect("failed to prepare snapshot list query");
        let rows = stmt
            .query_map(params![file_id], |row| {
                let date_str: String = row.get(1)?;
                Ok(SnapshotEntity {
                    id: row.get(0)?,
                    date: parse_datetime(date_str),
                    content: row.get(2)?,
                })
            })
            .expect("failed to read snapshots");
        rows.map(|row| row.expect("failed to read snapshot row"))
            .collect()
    }
}
