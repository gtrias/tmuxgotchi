use std::fs;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::Path;

use serde::Deserialize;

/// Extracted info from a JSONL session file.
#[derive(Debug, Default, Clone)]
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

/// Parse a JSONL file, incrementally if possible.
pub fn parse_jsonl(
    path: &Path,
    prev_file_size: u64,
    prev_session_id: Option<String>,
    prev_total_tokens: u64,
    prev_cost: Option<f64>,
    prev_activity: Option<String>,
) -> JsonlInfo {
    let file = match fs::File::open(path) {
        Ok(f) => f,
        Err(_) => return JsonlInfo::default(),
    };

    let file_size = file.metadata().map(|m| m.len()).unwrap_or(0);

    // If file hasn't changed, return cached data
    if file_size == prev_file_size && prev_file_size > 0 {
        return JsonlInfo {
            session_id: prev_session_id,
            total_tokens: prev_total_tokens,
            total_cost: prev_cost,
            last_activity: prev_activity,
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
    if prev_file_size > 0 {
        let _ = reader.seek(SeekFrom::Start(prev_file_size));
        session_id = prev_session_id;
        total_tokens = prev_total_tokens;
        total_cost = prev_cost.unwrap_or(0.0);
        last_activity = prev_activity;
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
