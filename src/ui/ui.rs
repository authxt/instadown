use ratatui::{
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs},
    Frame,
};

use super::app::{App, DownloadStatus, InputMode, FocusedArea};

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
            Constraint::Length(9),  // Exit button (width of "[Exit]" + borders)
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
    let exit_text = "[Exit]";
    let is_focused = matches!(app.focused_area, FocusedArea::ExitButton);
    
    let exit_button = Paragraph::new(exit_text)
        .alignment(Alignment::Center)
        .style(
            if is_focused {
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Gray)
            }
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(
                    if is_focused {
                        Style::default().fg(Color::Red)
                    } else {
                        Style::default()
                    }
                )
        );

    frame.render_widget(exit_button, area);
}

fn render_tabs(frame: &mut Frame, app: &App, area: Rect) {
    let titles: Vec<Line> = vec!["Download", "History"]
        .iter()
        .map(|t| Line::from(Span::styled(*t, Style::default().fg(Color::White))))
        .collect();

    let tabs = Tabs::new(titles)
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Tabs")
            .border_style(if matches!(app.focused_area, FocusedArea::Tabs) {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            }))
        .select(app.selected_tab)
        .highlight_style(Style::default()
            .add_modifier(Modifier::BOLD)
            .bg(Color::DarkGray));

    frame.render_widget(tabs, area);
}

fn render_input(frame: &mut Frame, app: &App, area: Rect) {
    let is_focused = matches!(app.focused_area, FocusedArea::Input);
    let input = Paragraph::new(app.input.as_str())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Instagram URL")
            .border_style(if is_focused {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            }));

    frame.render_widget(input, area);

    if let InputMode::Editing = app.input_mode {
        frame.set_cursor(
            area.x + app.input.len() as u16 + 1,
            area.y + 1,
        );
    }
}

fn render_download_tab(frame: &mut Frame, app: &App, area: Rect) {
    let download_block = Block::default()
        .borders(Borders::ALL)
        .title("Download Status");

    let status_text = match &app.download_status {
        DownloadStatus::None => "Press 'i' to enter URL, 'Enter' to download, or click to interact".to_string(),
        DownloadStatus::InProgress => "Starting download...".to_string(),
        DownloadStatus::Complete => "Download complete!".to_string(),
        DownloadStatus::Error(err) => format!("Error: {}", err),
    };

    let status = Paragraph::new(status_text)
        .style(Style::default().fg(match app.download_status {
            DownloadStatus::None => Color::White,
            DownloadStatus::InProgress => Color::Yellow,
            DownloadStatus::Complete => Color::Green,
            DownloadStatus::Error(_) => Color::Red,
        }))
        .block(download_block);

    frame.render_widget(status, area);
}

fn render_history_tab(frame: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .downloads
        .iter()
        .enumerate()
        .map(|(i, download)| {
            let style = if Some(i) == app.selected_history_item {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            ListItem::new(Line::from(vec![
                Span::styled(
                    download.timestamp.format("%Y-%m-%d %H:%M:%S").to_string(),
                    style.fg(Color::Gray),
                ),
                Span::raw(" "),
                Span::styled(
                    &download.filename,
                    style,
                ),
            ]))
        })
        .collect();

    let history = List::new(items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Download History")
            .border_style(if matches!(app.focused_area, FocusedArea::History) {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            }));

    frame.render_widget(history, area);
} 