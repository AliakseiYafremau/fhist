use chrono::{DateTime, Utc};

pub fn output_file_info(file_id: &str, file_path: &str) {
    println!("FILE\n\tID   | {}\n\tPATH | {}", file_id, file_path);
}

pub fn output_snapshot_info(snapshot_id: &str, snapshot_date: DateTime<Utc>, snapshot_contet: &str) {
    println!("Snapshot: id - {}, time - {}, content - {}", snapshot_id, snapshot_date, snapshot_contet);
}