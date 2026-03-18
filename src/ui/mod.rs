mod table;
mod tamagotchi;

use ratatui::Frame;

use crate::app::{App, ViewMode};

pub fn render(frame: &mut Frame, app: &mut App) {
    match app.view_mode {
        ViewMode::Table => table::render(frame, app),
        ViewMode::Tamagotchi => tamagotchi::render(frame, app),
    }
}
