use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent};

use crate::debug_log;
use crate::session::{self, PiSession, SessionStatus};
use crate::tmux;

#[derive(Clone, Copy, PartialEq)]
pub enum ViewMode {
    Table,
    Tamagotchi,
}

pub struct App {
    pub sessions: Vec<PiSession>,
    pub selected: usize,
    pub should_quit: bool,
    pub view_mode: ViewMode,
    pub tick: u64,
    pub tama_page: usize,
    pub tama_zoomed_room: Option<String>,
    pub tama_selected_agent: usize,
    prev_sessions: HashMap<String, PiSession>,
}

impl App {
    pub fn new() -> Self {
        App {
            sessions: Vec::new(),
            selected: 0,
            should_quit: false,
            view_mode: ViewMode::Table,
            tick: 0,
            tama_page: 0,
            tama_zoomed_room: None,
            tama_selected_agent: 0,
            prev_sessions: HashMap::new(),
        }
    }

    pub fn refresh(&mut self) {
        debug_log!("=== REFRESH START ===");
        debug_log!("prev_sessions count: {}", self.prev_sessions.len());

        let sessions = session::discover_sessions(&self.prev_sessions);

        debug_log!("new sessions count: {}", sessions.len());
        for s in &sessions {
            debug_log!("  - {} ({}) -> {}", s.project_name, s.session_id, s.status.label());
        }

        self.prev_sessions = sessions
            .iter()
            .map(|s| (s.session_id.clone(), s.clone()))
            .collect();

        self.sessions = sessions;
        debug_log!("=== REFRESH END ===");

        if self.selected >= self.sessions.len() && !self.sessions.is_empty() {
            self.selected = self.sessions.len() - 1;
        }
    }

    pub fn advance_tick(&mut self) {
        self.tick = self.tick.wrapping_add(1);
    }

    pub fn find_next_working(&self) -> Option<&PiSession> {
        self.sessions
            .iter()
            .find(|s| matches!(s.status, SessionStatus::Working | SessionStatus::Input))
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        // Global: jump to next working/input
        if matches!(key.code, KeyCode::Tab | KeyCode::Char('i')) {
            if let Some(session) = self.find_next_working() {
                tmux::switch_to_session(&session.tmux_pane);
                self.should_quit = true;
            }
            return;
        }

        match self.view_mode {
            ViewMode::Table => self.handle_key_table(key),
            ViewMode::Tamagotchi => self.handle_key_tamagotchi(key),
        }
    }

    fn handle_key_table(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
            KeyCode::Char('v') => self.view_mode = ViewMode::Tamagotchi,
            KeyCode::Char('j') | KeyCode::Down => {
                if !self.sessions.is_empty() {
                    self.selected = (self.selected + 1).min(self.sessions.len() - 1);
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                if self.selected > 0 {
                    self.selected -= 1;
                }
            }
            KeyCode::Enter => {
                if let Some(session) = self.sessions.get(self.selected) {
                    tmux::switch_to_session(&session.tmux_pane);
                    self.should_quit = true;
                }
            }
            KeyCode::Char('x') => {
                if let Some(session) = self.sessions.get(self.selected) {
                    tmux::kill_pane(&session.tmux_pane);
                    self.refresh();
                }
            }
            KeyCode::Char('n') => {
                let cwd = std::env::current_dir()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|_| ".".to_string());
                if let Ok(name) = tmux::create_session(&cwd) {
                    tmux::switch_to_session(&name);
                    self.should_quit = true;
                }
            }
            KeyCode::Char('r') => self.refresh(),
            _ => {}
        }
    }

    fn handle_key_tamagotchi(&mut self, key: KeyEvent) {
        // When zoomed into a room
        if self.tama_zoomed_room.is_some() {
            match key.code {
                KeyCode::Char('l') | KeyCode::Right => {
                    self.tama_selected_agent = self.tama_selected_agent.saturating_add(1);
                    return;
                }
                KeyCode::Char('h') | KeyCode::Left => {
                    self.tama_selected_agent = self.tama_selected_agent.saturating_sub(1);
                    return;
                }
                KeyCode::Enter => {
                    if let Some(session) = self.selected_zoomed_session() {
                        let pane = session.tmux_pane.clone();
                        tmux::switch_to_session(&pane);
                        self.should_quit = true;
                    }
                    return;
                }
                KeyCode::Char('x') => {
                    if let Some(session) = self.selected_zoomed_session() {
                        let pane = session.tmux_pane.clone();
                        tmux::kill_pane(&pane);
                        self.refresh();
                    }
                    return;
                }
                KeyCode::Char('n') => {
                    if let Some(cwd) = self.zoomed_room_cwd() {
                        if let Ok(name) = tmux::create_session(&cwd) {
                            tmux::switch_to_session(&name);
                            self.should_quit = true;
                        }
                    }
                    return;
                }
                _ => {}
            }
        }

        match key.code {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Esc => {
                if self.tama_zoomed_room.is_some() {
                    self.tama_zoomed_room = None;
                    self.tama_selected_agent = 0;
                } else {
                    self.should_quit = true;
                }
            }
            KeyCode::Char('v') => {
                self.tama_zoomed_room = None;
                self.tama_selected_agent = 0;
                self.view_mode = ViewMode::Table;
            }
            KeyCode::Char('r') => self.refresh(),
            KeyCode::Char('j') | KeyCode::Down => {
                self.tama_page = self.tama_page.saturating_add(1);
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.tama_page = self.tama_page.saturating_sub(1);
            }
            KeyCode::Char(c @ '1'..='9') => {
                let room_idx = (c as usize) - ('1' as usize);
                let rooms = self.get_rooms();
                let start = self.tama_page * 4;
                if let Some(room_name) = rooms.get(start + room_idx) {
                    self.tama_zoomed_room = Some(room_name.clone());
                    self.tama_selected_agent = 0;
                }
            }
            _ => {}
        }
    }

    pub fn get_rooms(&self) -> Vec<String> {
        let mut rooms: Vec<String> = self
            .sessions
            .iter()
            .map(|s| shorten_home(&s.cwd))
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        rooms.sort();
        rooms
    }

    fn zoomed_room_session_indices(&self) -> Vec<usize> {
        let Some(ref room_name) = self.tama_zoomed_room else {
            return Vec::new();
        };
        self.sessions
            .iter()
            .enumerate()
            .filter(|(_, s)| shorten_home(&s.cwd) == *room_name)
            .map(|(i, _)| i)
            .collect()
    }

    fn selected_zoomed_session(&self) -> Option<&PiSession> {
        let indices = self.zoomed_room_session_indices();
        if indices.is_empty() {
            return None;
        }
        let clamped = self.tama_selected_agent.min(indices.len() - 1);
        self.sessions.get(indices[clamped])
    }

    fn zoomed_room_cwd(&self) -> Option<String> {
        self.selected_zoomed_session().map(|s| s.cwd.clone())
    }

    pub fn to_json(&self) -> String {
        let sessions: Vec<serde_json::Value> = self
            .sessions
            .iter()
            .enumerate()
            .map(|(i, s)| {
                serde_json::json!({
                    "index": i + 1,
                    "session_id": s.session_id,
                    "project_name": s.project_name,
                    "branch": s.branch,
                    "cwd": s.cwd,
                    "tmux_pane": s.tmux_pane,
                    "model": s.model,
                    "think_level": s.think_level,
                    "context_pct": s.context_pct,
                    "context_window": s.context_window,
                    "cache_in": s.cache_in,
                    "is_subagent": s.is_subagent,
                    "has_lsp": s.has_lsp,
                    "status": s.status.label(),
                    "pid": s.pid,
                    "last_activity": s.last_activity,
                    "total_cost": s.total_cost,
                })
            })
            .collect();

        serde_json::to_string_pretty(&serde_json::json!({ "sessions": sessions }))
            .unwrap_or_else(|_| "{}".to_string())
    }
}

fn shorten_home(path: &str) -> String {
    if let Some(home) = dirs::home_dir() {
        let home_str = home.to_string_lossy();
        if let Some(rest) = path.strip_prefix(home_str.as_ref()) {
            return format!("~{rest}");
        }
    }
    path.to_string()
}
