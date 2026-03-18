use std::io;
use std::time::Duration;

use clap::{Parser, Subcommand};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

mod app;
mod jsonl;
mod model;
mod session;
mod sprites;
mod status_bar;
mod tmux;
mod ui;

use app::{App, ViewMode};

#[derive(Parser)]
#[command(name = "tmuxgotchi")]
#[command(about = "Tamagotchi-style TUI for managing pi agent sessions in tmux")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Show Tamagotchi view (creatures in rooms)
    View,
    /// Output session data as JSON
    Json,
    /// Launch a new pi session in tmux
    Launch {
        /// Working directory for new session
        #[arg(long)]
        cwd: Option<String>,
        /// Just print session name, don't attach
        #[arg(long)]
        name_only: bool,
    },
    /// Jump to next working/input agent
    Next,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Json) => {
            let mut app = App::new();
            app.refresh();
            println!("{}", app.to_json());
            Ok(())
        }
        Some(Commands::Launch { cwd, name_only }) => {
            let work_dir = cwd.unwrap_or_else(|| {
                std::env::current_dir()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|_| ".".to_string())
            });
            match tmux::create_session(&work_dir) {
                Ok(name) => {
                    if name_only {
                        println!("{}", name);
                    } else {
                        tmux::switch_to_session(&name);
                    }
                    Ok(())
                }
                Err(e) => {
                    eprintln!("Failed to launch session: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Some(Commands::Next) => {
            let mut app = App::new();
            app.refresh();
            if let Some(session) = app.find_next_working() {
                tmux::switch_to_session(&session.tmux_pane);
            }
            Ok(())
        }
        Some(Commands::View) => run_tui(ViewMode::Tamagotchi),
        None => run_tui(ViewMode::Table),
    }
}

fn run_tui(initial_view: ViewMode) -> io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = App::new();
    app.view_mode = initial_view;
    app.refresh();

    // Main loop
    let tick_rate = Duration::from_millis(200);
    let mut last_refresh = std::time::Instant::now();
    let refresh_interval = Duration::from_secs(2);

    loop {
        terminal.draw(|f| ui::render(f, &app))?;

        if event::poll(tick_rate)? {
            if let Event::Key(key) = event::read()? {
                app.handle_key(key);
            }
        }

        // Auto-refresh every 2 seconds
        if last_refresh.elapsed() >= refresh_interval {
            app.refresh();
            last_refresh = std::time::Instant::now();
        }

        // Animation tick
        app.advance_tick();

        if app.should_quit {
            break;
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
