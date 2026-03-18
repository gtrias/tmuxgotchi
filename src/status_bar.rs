use regex::Regex;
use std::process::Command;

/// Parsed information from pi's status bar.
#[derive(Debug, Default, Clone)]
pub struct StatusBarInfo {
    pub model: Option<String>,
    pub think_level: Option<String>,
    pub project: Option<String>,
    pub branch: Option<String>,
    pub git_dirty: Option<i32>,
    pub git_staged: Option<i32>,
    pub context_pct: Option<f32>,
    pub context_window: Option<String>,
    pub cache_in: Option<String>,
    pub is_subagent: bool,
    pub has_lsp: bool,
    pub has_cron: bool,
    pub has_heartbeat: bool,
}

/// Parse the status bar from a tmux pane.
pub fn parse_from_pane(pane_id: &str) -> StatusBarInfo {
    let output = Command::new("tmux")
        .args(["capture-pane", "-t", pane_id, "-p"])
        .output();

    let content = match output {
        Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout).to_string(),
        _ => return StatusBarInfo::default(),
    };

    // Find the status bar line (contains π and ├)
    for line in content.lines() {
        if line.contains('π') && line.contains('├') {
            return parse_status_line(line);
        }
    }

    StatusBarInfo::default()
}

/// Parse a pi status bar line.
///
/// Example formats:
/// π  ├ ◈ Opus 4.6 ├ think:off ├ 📁 workspace ├ ⎇ d14ba4b *2 +1 ├ ◫ 10.4%/1.0M ⚡ ├ cache in: 2.6M ├ (sub) ├ LSP
/// π  ├ ◈ Opus 4.6 ├ think:min ├ 📁 sunflare ├ ⎇ main ├ ◫ 24%/1M ├ cache in: 33M ├ LSP typescript
pub fn parse_status_line(line: &str) -> StatusBarInfo {
    let mut info = StatusBarInfo::default();

    // Model: ◈ <model>
    if let Some(caps) = Regex::new(r"◈\s*([^├]+)")
        .ok()
        .and_then(|re| re.captures(line))
    {
        info.model = Some(caps[1].trim().to_string());
    }

    // Think level: think:<level>
    if let Some(caps) = Regex::new(r"think:(\w+)")
        .ok()
        .and_then(|re| re.captures(line))
    {
        info.think_level = Some(caps[1].to_string());
    }

    // Project: 📁 <name>
    if let Some(caps) = Regex::new(r"📁\s*(\S+)")
        .ok()
        .and_then(|re| re.captures(line))
    {
        info.project = Some(caps[1].to_string());
    }

    // Branch: ⎇ <branch> [*N] [+N]
    if let Some(caps) = Regex::new(r"⎇\s*([^\s├]+)")
        .ok()
        .and_then(|re| re.captures(line))
    {
        info.branch = Some(caps[1].to_string());
    }

    // Git dirty: *N
    if let Some(caps) = Regex::new(r"\*(\d+)")
        .ok()
        .and_then(|re| re.captures(line))
    {
        info.git_dirty = caps[1].parse().ok();
    }

    // Git staged: +N
    if let Some(caps) = Regex::new(r"\+(\d+)")
        .ok()
        .and_then(|re| re.captures(line))
    {
        info.git_staged = caps[1].parse().ok();
    }

    // Context: ◫ <pct>%/<window>
    if let Some(caps) = Regex::new(r"◫\s*([\d.]+)%/([\d.]+[kKmM])")
        .ok()
        .and_then(|re| re.captures(line))
    {
        info.context_pct = caps[1].parse().ok();
        info.context_window = Some(caps[2].to_string());
    }

    // Cache: cache in: <size>
    if let Some(caps) = Regex::new(r"cache\s*in:\s*([\d.]+[kKmMgG]?)")
        .ok()
        .and_then(|re| re.captures(line))
    {
        info.cache_in = Some(caps[1].to_string());
    }

    // Flags
    info.is_subagent = line.contains("(sub)");
    info.has_lsp = line.contains("LSP");
    info.has_cron = line.contains("⏰") || line.contains("cron");
    info.has_heartbeat = line.contains("🫀") || line.contains("heartbeat");

    info
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_full_status_line() {
        let line = " π  ├ ◈ Opus 4.6 ├ think:off ├ 📁 workspace ├ ⎇ d14ba4b *2 +1 ├ ◫ 10.4%/1.0M ⚡ ├ cache in: 2.6M ├ (sub) ├ ⏰ cron active  ·  🫀 heartbeat active  ·  📊 Ticketdoor  ·  LSP";

        let info = parse_status_line(line);

        assert_eq!(info.model, Some("Opus 4.6".to_string()));
        assert_eq!(info.think_level, Some("off".to_string()));
        assert_eq!(info.project, Some("workspace".to_string()));
        assert_eq!(info.branch, Some("d14ba4b".to_string()));
        assert_eq!(info.git_dirty, Some(2));
        assert_eq!(info.git_staged, Some(1));
        assert_eq!(info.context_pct, Some(10.4));
        assert_eq!(info.context_window, Some("1.0M".to_string()));
        assert_eq!(info.cache_in, Some("2.6M".to_string()));
        assert!(info.is_subagent);
        assert!(info.has_lsp);
        assert!(info.has_cron);
        assert!(info.has_heartbeat);
    }

    #[test]
    fn test_parse_minimal_status_line() {
        let line = " π  ├ ◈ Sonnet 4.6 ├ think:min ├ 📁 myproject ├ ⎇ main ├ ◫ 5%/200k ├ LSP";

        let info = parse_status_line(line);

        assert_eq!(info.model, Some("Sonnet 4.6".to_string()));
        assert_eq!(info.think_level, Some("min".to_string()));
        assert_eq!(info.project, Some("myproject".to_string()));
        assert_eq!(info.branch, Some("main".to_string()));
        assert_eq!(info.context_pct, Some(5.0));
        assert_eq!(info.context_window, Some("200k".to_string()));
        assert!(!info.is_subagent);
        assert!(info.has_lsp);
        assert!(!info.has_cron);
    }
}
