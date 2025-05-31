use ratatui::{
    layout::Rect,
    style::{Style, Modifier},
    text::{Line, Span},
    widgets::{Block, Borders, BorderType, Tabs},
    Frame,
};

use crate::ui::app::{App, FocusedArea};
use crate::ui::styles::{THEME_PRIMARY, THEME_SECONDARY, THEME_ACCENT, THEME_TEXT};

pub fn render_tabs(frame: &mut Frame, app: &App, area: Rect) {
    let titles: Vec<Line> = vec!["📥 Download", "📋 History"]
        .iter()
        .map(|t| {
            Line::from(vec![
                Span::styled(
                    *t,
                    Style::default()
                        .fg(THEME_TEXT)
                        .add_modifier(Modifier::BOLD)
                )
            ])
        })
        .collect();

    let tabs = Tabs::new(titles)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(
                if matches!(app.focused_area, FocusedArea::Tabs) {
                    Style::default().fg(THEME_PRIMARY)
                } else {
                    Style::default().fg(THEME_SECONDARY)
                }
            )
            .title(" Tabs "))
        .select(app.selected_tab)
        .highlight_style(
            Style::default()
                .fg(THEME_ACCENT)
                .add_modifier(Modifier::BOLD)
        );

    frame.render_widget(tabs, area);
} 