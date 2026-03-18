use std::process::Command;

/// Info about a tmux pane running pi.
pub struct TmuxPane {
    pub pid: i32,
    pub session_name: String,
    pub pane_id: String, // "session:window.pane"
    pub cwd: String,
}

/// Discover all tmux panes running pi.
pub fn discover_pi_panes() -> Vec<TmuxPane> {
    let output = match Command::new("tmux")
        .args([
            "list-panes",
            "-a",
            "-F",
            "#{pane_pid}|||#{session_name}|||#{window_index}|||#{pane_index}|||#{pane_current_command}|||#{pane_current_path}",
        ])
        .output()
    {
        Ok(o) if o.status.success() => o,
        _ => return vec![],
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut results = Vec::new();

    for line in stdout.lines() {
        let parts: Vec<&str> = line.splitn(6, "|||").collect();
        if parts.len() < 6 {
            continue;
        }

        let pid: i32 = match parts[0].parse() {
            Ok(p) => p,
            Err(_) => continue,
        };
        let session_name = parts[1];
        let window_idx = parts[2];
        let pane_idx = parts[3];
        let command = parts[4];
        let pane_path = parts[5];

        let pane_id = format!("{}:{}.{}", session_name, window_idx, pane_idx);

        // Check if this pane is running pi
        if command == "pi" {
            results.push(TmuxPane {
                pid,
                session_name: session_name.to_string(),
                pane_id,
                cwd: pane_path.to_string(),
            });
        } else if command == "bash" || command == "sh" || command == "zsh" || command == "fish" {
            // Check for pi child process
            if let Some(pi_pid) = find_pi_child(pid) {
                results.push(TmuxPane {
                    pid: pi_pid,
                    session_name: session_name.to_string(),
                    pane_id,
                    cwd: pane_path.to_string(),
                });
            }
        }
    }

    results
}

/// Find a pi child process of the given parent PID.
fn find_pi_child(parent_pid: i32) -> Option<i32> {
    let output = Command::new("pgrep")
        .args(["-P", &parent_pid.to_string()])
        .output()
        .ok()?;

    for line in String::from_utf8_lossy(&output.stdout).lines() {
        if let Ok(child_pid) = line.trim().parse::<i32>() {
            // Check if this child is pi
            if is_pi_process(child_pid) {
                return Some(child_pid);
            }
        }
    }
    None
}

/// Check if a PID is a pi process.
fn is_pi_process(pid: i32) -> bool {
    let output = Command::new("ps")
        .args(["-p", &pid.to_string(), "-o", "comm="])
        .output();

    match output {
        Ok(o) if o.status.success() => {
            let comm = String::from_utf8_lossy(&o.stdout).trim().to_string();
            comm == "pi" || comm.contains("pi")
        }
        _ => false,
    }
}

/// Switch to a tmux pane (inside tmux) or attach to it (outside tmux).
pub fn switch_to_session(pane_id: &str) {
    let inside_tmux = std::env::var("TMUX").is_ok();

    if inside_tmux {
        // Select the pane
        let _ = Command::new("tmux")
            .args(["select-pane", "-t", pane_id])
            .status();
        // Switch to the window containing this pane
        let _ = Command::new("tmux")
            .args(["select-window", "-t", pane_id])
            .status();
    } else {
        // Extract session name from pane_id
        let session = pane_id.split(':').next().unwrap_or(pane_id);
        let _ = Command::new("tmux")
            .args(["attach-session", "-t", session])
            .status();
    }
}

/// Kill a tmux pane.
pub fn kill_pane(pane_id: &str) -> bool {
    Command::new("tmux")
        .args(["kill-pane", "-t", pane_id])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Create a new tmux session with pi running in the given directory.
pub fn create_session(cwd: &str) -> Result<String, String> {
    let base_name = derive_session_name(cwd);
    let session_name = unique_session_name(&base_name);

    // Find pi binary
    let pi_path = which_pi().unwrap_or_else(|| "pi".to_string());

    let status = Command::new("tmux")
        .args([
            "new-session",
            "-d",
            "-s",
            &session_name,
            "-c",
            cwd,
            &pi_path,
        ])
        .status()
        .map_err(|e| format!("Failed to create tmux session: {}", e))?;

    if !status.success() {
        return Err("tmux new-session failed".to_string());
    }

    Ok(session_name)
}

fn derive_session_name(cwd: &str) -> String {
    std::path::Path::new(cwd)
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "pi".to_string())
        .replace('.', "-")
        .replace(':', "-")
}

fn unique_session_name(base_name: &str) -> String {
    if !session_exists(base_name) {
        return base_name.to_string();
    }

    let mut n = 2;
    loop {
        let candidate = format!("{}-{}", base_name, n);
        if !session_exists(&candidate) {
            return candidate;
        }
        n += 1;
    }
}

fn session_exists(name: &str) -> bool {
    Command::new("tmux")
        .args(["has-session", "-t", name])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn which_pi() -> Option<String> {
    let output = Command::new("which").arg("pi").output().ok()?;
    let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if path.is_empty() {
        None
    } else {
        Some(path)
    }
}
