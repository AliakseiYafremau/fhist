use chrono::{DateTime, Utc};

pub fn output_file_info(file_id: &str, file_path: &str) {
    println!("File");
    println!("  ID:   {}", file_id);
    println!("  Path: {}", file_path);
}

pub fn output_snapshot_info(
    snapshot_id: &str,
    snapshot_date: DateTime<Utc>,
    snapshot_content: &str,
) {
    println!("Snapshot");
    println!("  ID:   {}", snapshot_id);
    println!("  Time: {}", snapshot_date);
    output_content(snapshot_content);
}

fn output_content(content: &str) {
    let trimmed = content.trim_end_matches('\n');
    if trimmed.is_empty() {
        println!("  Content: <empty>");
        return;
    }

    let clipped = clip_content(trimmed, 400);
    let mut lines = clipped.lines();
    if let Some(first) = lines.next() {
        println!("  Content: {}", first);
        for line in lines {
            println!("           {}", line);
        }
    }
}

fn clip_content(content: &str, max_len: usize) -> String {
    if content.len() <= max_len {
        return content.to_string();
    }
    let mut clipped = content.chars().take(max_len).collect::<String>();
    clipped.push_str("...");
    clipped
}
