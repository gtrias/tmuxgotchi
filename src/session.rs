use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{Duration, SystemTime};

use crate::jsonl;
use crate::status_bar;
use crate::tmux;

#[derive(Debug, Clone, PartialEq)]
pub enum SessionStatus {
    New,
    Working,
    Idle,
    Input,
}

impl SessionStatus {
    pub fn label(&self) -> &str {
        match self {
            SessionStatus::New => "New",
            SessionStatus::Working => "Working",
            SessionStatus::Idle => "Idle",
            SessionStatus::Input => "Input",
        }
    }
}

#[derive(Debug, Clone)]
pub struct PiSession {
    pub session_id: String,
    pub project_name: String,
    pub branch: Option<String>,
    pub git_dirty: Option<i32>,
    pub git_staged: Option<i32>,
    pub cwd: String,
    pub tmux_session: String,
    pub tmux_pane: String,
    pub model: Option<String>,
    pub think_level: Option<String>,
    pub context_pct: Option<f32>,
    pub context_window: Option<String>,
    pub cache_in: Option<String>,
    pub is_subagent: bool,
    pub has_lsp: bool,
    pub has_cron: bool,
    pub has_heartbeat: bool,
    pub status: SessionStatus,
    pub pid: i32,
    pub last_activity: Option<String>,
    pub total_cost: Option<f64>,
    pub jsonl_path: PathBuf,
    pub last_file_size: u64,
}

impl PiSession {
    pub fn context_display(&self) -> String {
        match (&self.context_pct, &self.context_window) {
            (Some(pct), Some(win)) => format!("{:.0}%/{}", pct, win),
            _ => "—".to_string(),
        }
    }

    pub fn cost_display(&self) -> String {
        match self.total_cost {
            Some(c) if c > 0.0 => format!("${:.2}", c),
            _ => "—".to_string(),
        }
    }

    pub fn model_display(&self) -> String {
        match &self.model {
            Some(m) => {
                let name = crate::model::display_name(m);
                match &self.think_level {
                    Some(t) if t != "off" => format!("{} ({})", name, t),
                    _ => name.to_string(),
                }
            }
            None => "—".to_string(),
        }
    }
}

/// Discover all pi sessions running in tmux panes.
pub fn discover_sessions(prev_sessions: &HashMap<String, PiSession>) -> Vec<PiSession> {
    let panes = tmux::discover_pi_panes();
    let mut sessions = Vec::new();

    for pane in panes {
        // Get status bar info via tmux capture-pane
        let status_info = status_bar::parse_from_pane(&pane.pane_id);

        // Detect working status from pane content
        let is_working = detect_working_status(&pane.pane_id);

        // Find the most recent JSONL for this CWD
        let (jsonl_path, jsonl_info) = find_jsonl_for_cwd(&pane.cwd, prev_sessions);

        // Determine session status
        let status = if jsonl_info.total_tokens == 0 {
            SessionStatus::New
        } else if is_working {
            SessionStatus::Working
        } else {
            SessionStatus::Idle
        };

        // Get project name from status bar or derive from CWD
        let project_name = status_info
            .project
            .clone()
            .unwrap_or_else(|| derive_project_name(&pane.cwd));

        let session_id = jsonl_info.session_id.unwrap_or_else(|| {
            format!("tmux-{}-{}", pane.session_name, pane.pane_id.replace(':', "-"))
        });

        sessions.push(PiSession {
            session_id,
            project_name,
            branch: status_info.branch.clone(),
            git_dirty: status_info.git_dirty,
            git_staged: status_info.git_staged,
            cwd: pane.cwd.clone(),
            tmux_session: pane.session_name.clone(),
            tmux_pane: pane.pane_id.clone(),
            model: status_info.model.clone(),
            think_level: status_info.think_level.clone(),
            context_pct: status_info.context_pct,
            context_window: status_info.context_window.clone(),
            cache_in: status_info.cache_in.clone(),
            is_subagent: status_info.is_subagent,
            has_lsp: status_info.has_lsp,
            has_cron: status_info.has_cron,
            has_heartbeat: status_info.has_heartbeat,
            status,
            pid: pane.pid,
            last_activity: jsonl_info.last_activity,
            total_cost: jsonl_info.total_cost,
            jsonl_path,
            last_file_size: jsonl_info.file_size,
        });
    }

    // Sort by last activity (most recent first)
    sessions.sort_by(|a, b| b.last_activity.cmp(&a.last_activity));
    sessions
}

/// Detect if the pane shows a working/thinking indicator.
fn detect_working_status(pane_id: &str) -> bool {
    let output = Command::new("tmux")
        .args(["capture-pane", "-t", pane_id, "-p"])
        .output();

    let content = match output {
        Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout).to_string(),
        _ => return false,
    };

    // Spinner characters used by pi
    let spinners = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];

    // Check entire content for spinner + Working pattern
    let has_spinner = spinners.iter().any(|&s| content.contains(s));
    let has_working = content.contains("Working...");
    let has_thinking = content.contains("Thinking");

    // Working if we have spinner AND (Working... or Thinking)
    has_spinner && (has_working || has_thinking)
}

/// Encode a CWD path to pi's session directory format.
/// /home/genar/src/sunflare -> --home-genar-src-sunflare--
fn encode_cwd_to_session_dir(cwd: &str) -> String {
    // Pi encodes paths by replacing / with - and wrapping with --
    // /home/genar/src/sunflare -> --home-genar-src-sunflare--
    let without_leading_slash = cwd.strip_prefix('/').unwrap_or(cwd);
    let encoded = without_leading_slash.replace('/', "-");
    format!("--{}--", encoded)
}

/// Find the most recent JSONL file for a given CWD.
fn find_jsonl_for_cwd(
    cwd: &str,
    prev_sessions: &HashMap<String, PiSession>,
) -> (PathBuf, jsonl::JsonlInfo) {
    let sessions_dir = match dirs::home_dir() {
        Some(h) => h.join(".pi").join("agent").join("sessions"),
        None => return (PathBuf::new(), jsonl::JsonlInfo::default()),
    };

    let encoded = encode_cwd_to_session_dir(cwd);
    let project_dir = sessions_dir.join(&encoded);

    if !project_dir.is_dir() {
        return (PathBuf::new(), jsonl::JsonlInfo::default());
    }

    // Find most recently modified JSONL
    let cutoff = SystemTime::now() - Duration::from_secs(7 * 24 * 3600); // Last 7 days

    let mut best: Option<(PathBuf, SystemTime)> = None;

    if let Ok(entries) = fs::read_dir(&project_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.extension().map(|e| e == "jsonl").unwrap_or(false) {
                continue;
            }

            let modified = path
                .metadata()
                .ok()
                .and_then(|m| m.modified().ok())
                .unwrap_or(SystemTime::UNIX_EPOCH);

            if modified < cutoff {
                continue;
            }

            if best.as_ref().map_or(true, |(_, t)| modified > *t) {
                best = Some((path, modified));
            }
        }
    }

    match best {
        Some((path, _)) => {
            // Check if we can reuse cached data
            let prev = prev_sessions.values().find(|s| s.jsonl_path == path);

            let info = jsonl::parse_jsonl(
                &path,
                prev.map(|s| s.last_file_size).unwrap_or(0),
                prev.and_then(|s| Some(s.session_id.clone())),
                prev.and_then(|s| s.total_cost),
                prev.and_then(|s| s.last_activity.clone()),
            );

            (path, info)
        }
        None => (PathBuf::new(), jsonl::JsonlInfo::default()),
    }
}

/// Derive project name from CWD path.
fn derive_project_name(cwd: &str) -> String {
    std::path::Path::new(cwd)
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| cwd.to_string())
}
