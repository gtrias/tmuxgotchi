use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::App;
use crate::debug_log;
use crate::session::{PiSession, SessionStatus};
use crate::sprites;

const ROOMS_PER_PAGE: usize = 4;

pub fn render(frame: &mut Frame, app: &App) {
    let chunks = Layout::vertical([Constraint::Min(1), Constraint::Length(1)]).split(frame.area());

    if app.tama_zoomed_room.is_some() {
        render_zoomed_room(frame, app, chunks[0]);
    } else {
        render_rooms(frame, app, chunks[0]);
    }

    render_footer(frame, app, chunks[1]);
}

fn render_rooms(frame: &mut Frame, app: &App, area: Rect) {
    let rooms = app.get_rooms();
    let start = app.tama_page * ROOMS_PER_PAGE;
    let end = (start + ROOMS_PER_PAGE).min(rooms.len());
    let visible_rooms: Vec<&String> = rooms.iter().skip(start).take(ROOMS_PER_PAGE).collect();

    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(
            " tmuxgotchi — page {}/{} ",
            app.tama_page + 1,
            (rooms.len() + ROOMS_PER_PAGE - 1) / ROOMS_PER_PAGE
        ));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    if visible_rooms.is_empty() {
        let msg = Paragraph::new("No pi sessions running. Press 'n' to start one.")
            .style(Style::default().fg(Color::DarkGray));
        frame.render_widget(msg, inner);
        return;
    }

    // 2x2 grid layout
    let rows = Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]).split(inner);

    for (row_idx, row_area) in rows.iter().enumerate() {
        let cols = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).split(*row_area);

        for (col_idx, col_area) in cols.iter().enumerate() {
            let room_idx = row_idx * 2 + col_idx;
            if room_idx < visible_rooms.len() {
                let room_name = visible_rooms[room_idx];
                let sessions: Vec<&PiSession> = app
                    .sessions
                    .iter()
                    .filter(|s| shorten_home(&s.cwd) == **room_name)
                    .collect();

                render_room(frame, room_name, &sessions, room_idx + 1, app.tick, *col_area);
            }
        }
    }
}

fn render_room(
    frame: &mut Frame,
    room_name: &str,
    sessions: &[&PiSession],
    room_num: usize,
    tick: u64,
    area: Rect,
) {
    debug_log!("RENDER_ROOM: {} sessions in room '{}'", sessions.len(), room_name);
    for s in sessions {
        let creature = sprites::creature_for_session(&s.session_id);
        debug_log!("  -> project={} status={:?} creature={}", 
            s.project_name, s.status, creature.name());
    }
    
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" {} {} ", room_num, room_name));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    if sessions.is_empty() {
        return;
    }

    // Render creatures side by side
    let creature_width = inner.width / sessions.len().max(1) as u16;
    for (i, session) in sessions.iter().enumerate() {
        let creature_area = Rect {
            x: inner.x + (i as u16 * creature_width),
            y: inner.y,
            width: creature_width.min(inner.width - i as u16 * creature_width),
            height: inner.height,
        };
        render_creature(frame, session, tick, creature_area);
    }
}

fn render_creature(frame: &mut Frame, session: &PiSession, tick: u64, area: Rect) {
    let creature = sprites::creature_for_session(&session.session_id);
    let frames = sprites::get_frames(creature, session.status.clone());
    let frame_idx = ((tick / 4) as usize) % frames.len();
    let sprite_lines = frames[frame_idx];

    let color = match session.status {
        SessionStatus::Working => Color::Green,
        SessionStatus::Idle => Color::DarkGray,
        SessionStatus::Input => Color::Yellow,
        SessionStatus::New => Color::Blue,
    };

    let mut lines: Vec<Line> = sprite_lines
        .iter()
        .map(|&l| Line::from(Span::styled(l, Style::default().fg(color))))
        .collect();

    // Add status info below creature
    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled(
            format!("{} ", creature.name()),
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            session.status.label(),
            Style::default().fg(color),
        ),
    ]));

    if let Some(ref model) = session.model {
        lines.push(Line::from(Span::styled(
            model.clone(),
            Style::default().fg(Color::Cyan),
        )));
    }

    lines.push(Line::from(Span::styled(
        session.context_display(),
        Style::default().fg(Color::DarkGray),
    )));

    let para = Paragraph::new(lines).style(Style::default().fg(color));
    frame.render_widget(para, area);
}

fn render_zoomed_room(frame: &mut Frame, app: &App, area: Rect) {
    let room_name = app.tama_zoomed_room.as_ref().unwrap();
    let sessions: Vec<&PiSession> = app
        .sessions
        .iter()
        .filter(|s| shorten_home(&s.cwd) == *room_name)
        .collect();

    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" {} ", room_name));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    if sessions.is_empty() {
        let msg = Paragraph::new("Room is empty").style(Style::default().fg(Color::DarkGray));
        frame.render_widget(msg, inner);
        return;
    }

    // Larger creature rendering when zoomed
    let creature_width = inner.width / sessions.len().max(1) as u16;
    for (i, session) in sessions.iter().enumerate() {
        let is_selected = i == app.tama_selected_agent.min(sessions.len() - 1);

        let creature_area = Rect {
            x: inner.x + (i as u16 * creature_width),
            y: inner.y,
            width: creature_width.min(inner.width - i as u16 * creature_width),
            height: inner.height,
        };

        render_zoomed_creature(frame, session, app.tick, is_selected, creature_area);
    }
}

fn render_zoomed_creature(
    frame: &mut Frame,
    session: &PiSession,
    tick: u64,
    is_selected: bool,
    area: Rect,
) {
    let creature = sprites::creature_for_session(&session.session_id);
    let frames = sprites::get_frames(creature, session.status.clone());
    let frame_idx = ((tick / 4) as usize) % frames.len();
    let sprite_lines = frames[frame_idx];

    let color = match session.status {
        SessionStatus::Working => Color::Green,
        SessionStatus::Idle => Color::DarkGray,
        SessionStatus::Input => Color::Yellow,
        SessionStatus::New => Color::Blue,
    };

    // Add some padding at the top
    let mut lines: Vec<Line> = vec![Line::from("")];
    
    // Render sprite
    for &line in sprite_lines {
        lines.push(Line::from(Span::styled(line, Style::default().fg(color))));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled(
            format!("{} ", creature.name()),
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            session.status.label(),
            Style::default().fg(color),
        ),
    ]));

    if let Some(ref model) = session.model {
        lines.push(Line::from(Span::styled(
            format!("Model: {}", model),
            Style::default().fg(Color::Cyan),
        )));
    }

    lines.push(Line::from(Span::styled(
        format!("Context: {}", session.context_display()),
        Style::default().fg(Color::White),
    )));

    if let Some(ref cache) = session.cache_in {
        lines.push(Line::from(Span::styled(
            format!("Cache: {}", cache),
            Style::default().fg(Color::DarkGray),
        )));
    }

    lines.push(Line::from(Span::styled(
        format!("Cost: {}", session.cost_display()),
        Style::default().fg(Color::Yellow),
    )));

    lines.push(Line::from(Span::styled(
        session.tmux_pane.clone(),
        Style::default().fg(Color::DarkGray),
    )));

    let style = if is_selected {
        Style::default().fg(color).bg(Color::Rgb(40, 40, 40))
    } else {
        Style::default().fg(color)
    };

    let para = Paragraph::new(lines).style(style);
    frame.render_widget(para, area);
}

fn render_footer(frame: &mut Frame, app: &App, area: Rect) {
    let footer = if app.tama_zoomed_room.is_some() {
        Paragraph::new(Line::from(vec![
            Span::styled("h/l", Style::default().fg(Color::Cyan)),
            Span::raw(" select  "),
            Span::styled("Enter", Style::default().fg(Color::Cyan)),
            Span::raw(" switch  "),
            Span::styled("x", Style::default().fg(Color::Cyan)),
            Span::raw(" kill  "),
            Span::styled("n", Style::default().fg(Color::Cyan)),
            Span::raw(" new  "),
            Span::styled("Esc", Style::default().fg(Color::Cyan)),
            Span::raw(" back  "),
            Span::styled("v", Style::default().fg(Color::Cyan)),
            Span::raw(" table  "),
            Span::styled("q", Style::default().fg(Color::Cyan)),
            Span::raw(" quit"),
        ]))
    } else {
        Paragraph::new(Line::from(vec![
            Span::styled("1-4", Style::default().fg(Color::Cyan)),
            Span::raw(" zoom  "),
            Span::styled("j/k", Style::default().fg(Color::Cyan)),
            Span::raw(" page  "),
            Span::styled("v", Style::default().fg(Color::Cyan)),
            Span::raw(" table  "),
            Span::styled("r", Style::default().fg(Color::Cyan)),
            Span::raw(" refresh  "),
            Span::styled("q", Style::default().fg(Color::Cyan)),
            Span::raw(" quit"),
        ]))
    };

    frame.render_widget(footer, area);
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
