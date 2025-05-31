use ratatui::{
    layout::Rect,
    style::Style,
    widgets::{Block, Borders, BorderType, Paragraph},
    Frame,
};

use crate::ui::app::{App, FocusedArea, InputMode};
use crate::ui::styles::{THEME_PRIMARY, THEME_SECONDARY, THEME_ACCENT, THEME_TEXT};

pub fn render_input(frame: &mut Frame, app: &App, area: Rect) {
    let is_focused = matches!(app.focused_area, FocusedArea::Input);
    let input = Paragraph::new(app.input.as_str())
        .style(
            if matches!(app.input_mode, InputMode::Editing) {
                Style::default().fg(THEME_ACCENT)
            } else {
                Style::default().fg(THEME_TEXT)
            }
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(" 🔗 Instagram URL ")
                .border_style(
                    if is_focused {
                        Style::default().fg(THEME_PRIMARY)
                    } else {
                        Style::default().fg(THEME_SECONDARY)
                    }
                )
        );

    frame.render_widget(input, area);

    if let InputMode::Editing = app.input_mode {
        frame.set_cursor(
            area.x + app.input.len() as u16 + 1,
            area.y + 1,
        );
    }
} 