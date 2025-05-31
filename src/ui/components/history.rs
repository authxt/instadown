use ratatui::{
    layout::Rect,
    style::{Style, Modifier},
    text::{Line, Span},
    widgets::{Block, Borders, BorderType, List, ListItem},
    Frame,
};

use crate::ui::app::{App, FocusedArea};
use crate::ui::styles::{THEME_PRIMARY, THEME_SECONDARY, THEME_ACCENT, THEME_TEXT, THEME_WARNING};

pub fn render_history_tab(frame: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .downloads
        .iter()
        .enumerate()
        .map(|(i, download)| {
            let style = if Some(i) == app.selected_history_item {
                Style::default()
                    .fg(THEME_ACCENT)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(THEME_TEXT)
            };

            ListItem::new(Line::from(vec![
                Span::styled(
                    format!("📅 {} ", download.timestamp.format("%Y-%m-%d %H:%M:%S")),
                    style.fg(THEME_WARNING)
                ),
                Span::raw(" "),
                Span::styled(
                    format!("📹 {}", download.filename),
                    style
                ),
            ]))
        })
        .collect();

    let history = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(" 📜 Download History ")
                .border_style(
                    if matches!(app.focused_area, FocusedArea::Tabs) && app.selected_tab == 1 {
                        Style::default().fg(THEME_PRIMARY)
                    } else {
                        Style::default().fg(THEME_SECONDARY)
                    }
                )
        );

    frame.render_widget(history, area);
} 