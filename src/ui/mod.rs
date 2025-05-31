pub mod app;
pub mod components;
pub mod styles;

use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

use app::App;
use components::*;

pub fn render(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Tabs and Exit button
            Constraint::Length(3),  // Input
            Constraint::Min(0),     // Content
        ])
        .split(frame.size());

    // Create a horizontal layout for tabs and exit button
    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(20),    // Tabs
            Constraint::Length(9),  // Exit button
        ])
        .split(chunks[0]);

    render_tabs(frame, app, top_chunks[0]);
    render_exit_button(frame, app, top_chunks[1]);
    render_input(frame, app, chunks[1]);

    match app.selected_tab {
        0 => render_download_tab(frame, app, chunks[2]),
        1 => render_history_tab(frame, app, chunks[2]),
        _ => unreachable!(),
    }
} 