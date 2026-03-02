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

pub fn output_diff(
    file_id: &str,
    file_path: &str,
    from_id: &str,
    from_date: DateTime<Utc>,
    from_content: &str,
    to_id: &str,
    to_date: DateTime<Utc>,
    to_content: &str,
) {
    println!("[diff]");
    println!("file id: {file_id}");
    println!("file path: {file_path}");
    println!("from: {from_id} ({})", from_date.to_rfc3339());
    println!("to: {to_id} ({})", to_date.to_rfc3339());
    println!("--- from");
    println!("+++ to");
    for line in diff_lines(from_content, to_content) {
        match line.kind {
            DiffKind::Same => println!(" {}", line.value),
            DiffKind::Remove => println!("-{}", line.value),
            DiffKind::Add => println!("+{}", line.value),
        }
    }
}

pub fn output_rollback(file_id: &str, file_path: &str, snapshot_id: &str) {
    println!("[rollback]");
    println!("file id: {file_id}");
    println!("file path: {file_path}");
    println!("snapshot id: {snapshot_id}");
}

fn format_content(content: &str) -> String {
    let trimmed = content.trim_end_matches('\n');
    if trimmed.is_empty() {
        return "<empty>".to_string();
    }
    clip_content(trimmed, 400)
}

enum DiffKind {
    Same,
    Add,
    Remove,
}

struct DiffLine {
    kind: DiffKind,
    value: String,
}

fn diff_lines(from: &str, to: &str) -> Vec<DiffLine> {
    let from_lines: Vec<&str> = from.lines().collect();
    let to_lines: Vec<&str> = to.lines().collect();

    let mut dp = vec![vec![0usize; to_lines.len() + 1]; from_lines.len() + 1];
    for i in (0..from_lines.len()).rev() {
        for j in (0..to_lines.len()).rev() {
            if from_lines[i] == to_lines[j] {
                dp[i][j] = dp[i + 1][j + 1] + 1;
            } else {
                dp[i][j] = dp[i + 1][j].max(dp[i][j + 1]);
            }
        }
    }

    let mut result = Vec::new();
    let mut i = 0;
    let mut j = 0;
    while i < from_lines.len() && j < to_lines.len() {
        if from_lines[i] == to_lines[j] {
            result.push(DiffLine {
                kind: DiffKind::Same,
                value: from_lines[i].to_string(),
            });
            i += 1;
            j += 1;
        } else if dp[i + 1][j] >= dp[i][j + 1] {
            result.push(DiffLine {
                kind: DiffKind::Remove,
                value: from_lines[i].to_string(),
            });
            i += 1;
        } else {
            result.push(DiffLine {
                kind: DiffKind::Add,
                value: to_lines[j].to_string(),
            });
            j += 1;
        }
    }

    while i < from_lines.len() {
        result.push(DiffLine {
            kind: DiffKind::Remove,
            value: from_lines[i].to_string(),
        });
        i += 1;
    }

    while j < to_lines.len() {
        result.push(DiffLine {
            kind: DiffKind::Add,
            value: to_lines[j].to_string(),
        });
        j += 1;
    }

    result
}

fn clip_content(content: &str, max_len: usize) -> String {
    if content.len() <= max_len {
        return content.to_string();
    }
    let mut clipped = content.chars().take(max_len).collect::<String>();
    clipped.push_str("...");
    clipped
}
