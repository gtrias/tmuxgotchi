use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame,
};

use crate::app::App;
use crate::session::SessionStatus;

pub fn render(frame: &mut Frame, app: &App) {
    let chunks = Layout::vertical([Constraint::Min(1), Constraint::Length(1)]).split(frame.area());

    render_table(frame, app, chunks[0]);
    render_footer(frame, chunks[1]);
}

fn render_table(frame: &mut Frame, app: &App, area: Rect) {
    let header = Row::new(vec![
        Cell::from(" # "),
        Cell::from("Session"),
        Cell::from("Project::Branch"),
        Cell::from("Status"),
        Cell::from("Model"),
        Cell::from("Context"),
        Cell::from("Cost"),
    ])
    .style(
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    );

    let rows: Vec<Row> = app
        .sessions
        .iter()
        .enumerate()
        .map(|(i, session)| {
            let num = format!(" {} ", i + 1);

            // Status: colored dot + label
            let (status_dot, status_label, status_color) = match session.status {
                SessionStatus::New => ("●", "New", Color::Blue),
                SessionStatus::Working => ("●", "Work", Color::Green),
                SessionStatus::Idle => ("●", "Idle", Color::DarkGray),
                SessionStatus::Input => ("●", "Input", Color::Yellow),
            };

            // Project::Branch
            let project_cell = match &session.branch {
                Some(b) => Cell::from(Line::from(vec![
                    Span::raw(&session.project_name),
                    Span::styled("::", Style::default().fg(Color::DarkGray)),
                    Span::styled(b, Style::default().fg(Color::Green)),
                ])),
                None => Cell::from(session.project_name.clone()),
            };

            // Status cell
            let status_cell = Cell::from(Line::from(vec![
                Span::styled(status_dot, Style::default().fg(status_color)),
                Span::styled(format!(" {}", status_label), Style::default().fg(status_color)),
            ]));

            // Context with color based on usage
            let context_display = session.context_display();
            let context_style = match session.context_pct {
                Some(pct) if pct > 90.0 => Style::default().fg(Color::Red),
                Some(pct) if pct > 75.0 => Style::default().fg(Color::Yellow),
                _ => Style::default(),
            };

            let row = Row::new(vec![
                Cell::from(num),
                Cell::from(session.tmux_pane.clone()),
                project_cell,
                status_cell,
                Cell::from(session.model_display()),
                Cell::from(context_display).style(context_style),
                Cell::from(session.cost_display()),
            ]);

            // Highlight selected row or working sessions
            if session.status == SessionStatus::Input {
                row.style(Style::default().bg(Color::Rgb(50, 40, 0)))
            } else if session.status == SessionStatus::Working {
                row.style(Style::default().bg(Color::Rgb(0, 30, 0)))
            } else if i == app.selected {
                row.style(Style::default().bg(Color::DarkGray))
            } else {
                row
            }
        })
        .collect();

    let widths = [
        Constraint::Length(4),  // #
        Constraint::Length(16), // Session
        Constraint::Min(20),    // Project::Branch
        Constraint::Length(8),  // Status
        Constraint::Length(18), // Model
        Constraint::Length(12), // Context
        Constraint::Length(8),  // Cost
    ];

    let table = Table::new(rows, widths).header(header).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" tmuxgotchi "),
    );

    frame.render_widget(table, area);
}

fn render_footer(frame: &mut Frame, area: Rect) {
    let footer = Paragraph::new(Line::from(vec![
        Span::styled("j/k", Style::default().fg(Color::Cyan)),
        Span::raw(" navigate  "),
        Span::styled("Enter", Style::default().fg(Color::Cyan)),
        Span::raw(" switch  "),
        Span::styled("x", Style::default().fg(Color::Cyan)),
        Span::raw(" kill  "),
        Span::styled("v", Style::default().fg(Color::Cyan)),
        Span::raw(" tamagotchi  "),
        Span::styled("n", Style::default().fg(Color::Cyan)),
        Span::raw(" new  "),
        Span::styled("r", Style::default().fg(Color::Cyan)),
        Span::raw(" refresh  "),
        Span::styled("q", Style::default().fg(Color::Cyan)),
        Span::raw(" quit"),
    ]));

    frame.render_widget(footer, area);
}
