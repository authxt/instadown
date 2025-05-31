use ratatui::{
    prelude::*,
    widgets::{Block, Borders, BorderType, Paragraph},
};

use crate::ui::app::{App, FocusedArea};
use crate::ui::styles::THEME_ERROR;

pub fn render_exit_button(frame: &mut Frame, app: &App, area: Rect) {
    let exit_text = "❌ Exit";
    let is_focused = matches!(app.focused_area, FocusedArea::ExitButton);
    
    let exit_button = Paragraph::new(exit_text)
        .alignment(Alignment::Center)
        .style(
            if is_focused {
                Style::default().fg(THEME_ERROR).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Gray)
            }
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(
                    if is_focused {
                        Style::default().fg(THEME_ERROR)
                    } else {
                        Style::default().fg(Color::DarkGray)
                    }
                )
        );

    frame.render_widget(exit_button, area);
} 