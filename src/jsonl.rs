use std::fs;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::Path;

use serde::Deserialize;

/// Extracted info from a JSONL session file.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct JsonlInfo {
    pub session_id: Option<String>,
    pub total_tokens: u64,
    pub total_cost: Option<f64>,
    pub last_activity: Option<String>,
    pub file_size: u64,
}

/// Minimal serde structs for JSONL parsing.
#[derive(Deserialize)]
struct SessionEntry {
    #[serde(rename = "type")]
    entry_type: String,
    id: Option<String>,
    timestamp: Option<String>,
}

#[derive(Deserialize)]
struct MessageEntry {
    #[serde(rename = "type")]
    entry_type: String,
    timestamp: Option<String>,
    message: Option<MessageContent>,
}

#[derive(Deserialize)]
struct MessageContent {
    usage: Option<UsageInfo>,
}

#[derive(Deserialize)]
struct UsageInfo {
    #[serde(default)]
    input: u64,
    #[serde(default)]
    output: u64,
    #[serde(rename = "cacheRead", default)]
    cache_read: u64,
    #[serde(rename = "cacheWrite", default)]
    cache_write: u64,
    #[serde(rename = "totalTokens", default)]
    total_tokens: u64,
    cost: Option<CostInfo>,
}

#[derive(Deserialize)]
struct CostInfo {
    #[serde(default)]
    total: f64,
}

/// Parse a JSONL file, incrementally: `prev` is what a previous call reported
/// for the same file (or the default when it is new), and everything the file
/// has appended since is read on top of it.
pub fn parse_jsonl(path: &Path, prev: &JsonlInfo) -> JsonlInfo {
    let file = match fs::File::open(path) {
        Ok(f) => f,
        Err(_) => return JsonlInfo::default(),
    };

    let file_size = file.metadata().map(|m| m.len()).unwrap_or(0);

    // If file hasn't changed, return cached data
    if file_size == prev.file_size && prev.file_size > 0 {
        return JsonlInfo {
            session_id: prev.session_id.clone(),
            total_tokens: prev.total_tokens,
            total_cost: prev.total_cost,
            last_activity: prev.last_activity.clone(),
            file_size,
        };
    }

    let mut reader = BufReader::new(file);
    let mut session_id: Option<String> = None;
    let mut total_tokens: u64 = 0;
    let mut total_cost: f64 = 0.0;
    let mut last_activity: Option<String> = None;

    // If we have previous data and file grew, seek to where we left off
    // (but preserve session_id since it's only in the first line which we skip)
    if prev.file_size > 0 {
        let _ = reader.seek(SeekFrom::Start(prev.file_size));
        session_id = prev.session_id.clone();
        total_tokens = prev.total_tokens;
        total_cost = prev.total_cost.unwrap_or(0.0);
        last_activity = prev.last_activity.clone();
    }

    let mut line = String::new();
    loop {
        line.clear();
        match reader.read_line(&mut line) {
            Ok(0) => break, // EOF
            Ok(_) => {}
            Err(_) => break,
        }

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        // Parse session entry (first line)
        if trimmed.contains("\"type\":\"session\"") {
            if let Ok(entry) = serde_json::from_str::<SessionEntry>(trimmed) {
                session_id = entry.id;
            }
            continue;
        }

        // Parse message entries for usage/cost
        if trimmed.contains("\"type\":\"message\"") {
            if let Ok(entry) = serde_json::from_str::<MessageEntry>(trimmed) {
                if let Some(ts) = entry.timestamp {
                    last_activity = Some(ts);
                }

                if let Some(msg) = entry.message {
                    if let Some(usage) = msg.usage {
                        total_tokens = usage.total_tokens.max(usage.input + usage.output);

                        if let Some(cost) = usage.cost {
                            total_cost += cost.total;
                        }
                    }
                }
            }
        }
    }

    JsonlInfo {
        session_id,
        total_tokens,
        total_cost: if total_cost > 0.0 {
            Some(total_cost)
        } else {
            None
        },
        last_activity,
        file_size,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn write(path: &Path, contents: &str) {
        let mut f = fs::File::create(path).unwrap();
        f.write_all(contents.as_bytes()).unwrap();
    }

    fn dir() -> std::path::PathBuf {
        std::env::temp_dir().join(format!(
            "tmuxgotchi-jsonl-test-{}-{:?}",
            std::process::id(),
            std::time::SystemTime::now()
        ))
    }

    #[test]
    fn fresh_file_reports_session_usage_cost_and_activity() {
        let path = dir();
        write(
            &path,
            concat!(
                "{\"type\":\"session\",\"id\":\"sess-1\"}\n",
                "{\"type\":\"message\",\"timestamp\":\"2026-01-01T00:00:00Z\",\"message\":{\"usage\":{\"input\":10,\"output\":5,\"totalTokens\":15,\"cost\":{\"total\":0.25}}}}\n",
            ),
        );

        let info = parse_jsonl(&path, &JsonlInfo::default());

        assert_eq!(info.session_id.as_deref(), Some("sess-1"));
        assert_eq!(info.total_tokens, 15);
        assert_eq!(info.total_cost, Some(0.25));
        assert_eq!(info.last_activity.as_deref(), Some("2026-01-01T00:00:00Z"));
        assert!(info.file_size > 0);
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn unchanged_file_returns_the_previous_report() {
        let path = dir();
        write(
            &path,
            "{\"type\":\"session\",\"id\":\"sess-1\"}\n{\"type\":\"message\"}\n",
        );

        let first = parse_jsonl(&path, &JsonlInfo::default());
        let second = parse_jsonl(&path, &first);

        assert_eq!(second.session_id, first.session_id);
        assert_eq!(second.total_tokens, first.total_tokens);
        assert_eq!(second.total_cost, first.total_cost);
        assert_eq!(second.file_size, first.file_size);
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn appended_lines_add_onto_the_previous_report() {
        let path = dir();
        write(
            &path,
            concat!(
                "{\"type\":\"session\",\"id\":\"sess-1\"}\n",
                "{\"type\":\"message\",\"timestamp\":\"2026-01-01T00:00:00Z\",\"message\":{\"usage\":{\"input\":10,\"output\":5,\"totalTokens\":15,\"cost\":{\"total\":0.25}}}}\n",
            ),
        );
        let first = parse_jsonl(&path, &JsonlInfo::default());

        // Append a later message; the session line is never re-read.
        let mut f = fs::OpenOptions::new().append(true).open(&path).unwrap();
        writeln!(
            f,
            "{{\"type\":\"message\",\"timestamp\":\"2026-01-02T00:00:00Z\",\"message\":{{\"usage\":{{\"input\":20,\"output\":10,\"totalTokens\":30,\"cost\":{{\"total\":0.5}}}}}}}}"
        )
        .unwrap();

        let second = parse_jsonl(&path, &first);

        assert_eq!(second.session_id.as_deref(), Some("sess-1"));
        assert_eq!(second.total_tokens, 30);
        assert_eq!(second.total_cost, Some(0.75));
        assert_eq!(
            second.last_activity.as_deref(),
            Some("2026-01-02T00:00:00Z")
        );
        assert!(second.file_size > first.file_size);
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn missing_file_reports_nothing() {
        let path = dir();
        let info = parse_jsonl(&path, &JsonlInfo::default());
        assert_eq!(info, JsonlInfo::default());
    }
}
