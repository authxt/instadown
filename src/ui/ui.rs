use ratatui::{
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, BorderType, List, ListItem, Paragraph, Tabs, Wrap},
    Frame,
};

use super::app::{App, DownloadStatus, InputMode, FocusedArea};

const THEME_PRIMARY: Color = Color::Rgb(147, 112, 219);    // Light Purple
const THEME_SECONDARY: Color = Color::Rgb(106, 90, 205);   // Slate Blue
const THEME_ACCENT: Color = Color::Rgb(255, 105, 180);     // Hot Pink
const THEME_SUCCESS: Color = Color::Rgb(50, 205, 50);      // Lime Green
const THEME_ERROR: Color = Color::Rgb(255, 69, 0);         // Red Orange
const THEME_WARNING: Color = Color::Rgb(255, 215, 0);      // Gold
const THEME_TEXT: Color = Color::Rgb(248, 248, 255);       // Ghost White

pub fn render(frame: &mut Frame, app: &App) {
    // Create the main layout
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

fn render_exit_button(frame: &mut Frame, app: &App, area: Rect) {
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

fn render_tabs(frame: &mut Frame, app: &App, area: Rect) {
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

fn render_input(frame: &mut Frame, app: &App, area: Rect) {
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

fn render_download_tab(frame: &mut Frame, app: &App, area: Rect) {
    let (status_text, style) = match &app.download_status {
        DownloadStatus::None => (
            "✨ Press 'i' to enter URL, 'Enter' to download, or click to interact".to_string(),
            Style::default().fg(THEME_TEXT)
        ),
        DownloadStatus::InProgress => (
            "🚀 Starting download...".to_string(),
            Style::default().fg(THEME_WARNING)
        ),
        DownloadStatus::Downloading { progress, speed, eta, size } => (
            format!(
                "⬇️ Downloading... {:.1}%\n📊 Speed: {}\n⏱️ ETA: {}\n📦 Size: {}",
                progress * 100.0,
                speed,
                eta,
                size
            ),
            Style::default().fg(THEME_PRIMARY)
        ),
        DownloadStatus::Complete => (
            "✅ Download complete!".to_string(),
            Style::default().fg(THEME_SUCCESS)
        ),
        DownloadStatus::Error(err) => (
            format!("❌ Error: {}", err),
            Style::default().fg(THEME_ERROR)
        ),
    };

    let status = Paragraph::new(Text::from(status_text))
        .style(style)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .title(" Download Status ")
                .border_style(Style::default().fg(THEME_SECONDARY))
        )
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    frame.render_widget(status, area);
}

fn render_history_tab(frame: &mut Frame, app: &App, area: Rect) {
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
                    if matches!(app.focused_area, FocusedArea::History) {
                        Style::default().fg(THEME_PRIMARY)
                    } else {
                        Style::default().fg(THEME_SECONDARY)
                    }
                )
        );

    frame.render_widget(history, area);
} 