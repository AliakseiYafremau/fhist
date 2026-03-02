use chrono::{DateTime, Utc};

pub fn output_file_info(file_id: &str, file_path: &str) {
    println!("[file]");
    println!("id: {file_id}");
    println!("path: {file_path}");
}

pub fn output_snapshot_info(
    snapshot_id: &str,
    snapshot_date: DateTime<Utc>,
    snapshot_content: &str,
    verbose: bool,
) {
    println!("[snapshot]");
    println!("id: {snapshot_id}");
    println!("time: {}", snapshot_date.to_rfc3339());
    if verbose {
        let content = format_content(snapshot_content);
        let indented = content.replace('\n', "\n  ");
        println!("content:");
        println!("  {indented}");
    }
}

fn format_content(content: &str) -> String {
    let trimmed = content.trim_end_matches('\n');
    if trimmed.is_empty() {
        return "<empty>".to_string();
    }
    clip_content(trimmed, 400)
}

fn clip_content(content: &str, max_len: usize) -> String {
    if content.len() <= max_len {
        return content.to_string();
    }
    let mut clipped = content.chars().take(max_len).collect::<String>();
    clipped.push_str("...");
    clipped
}
