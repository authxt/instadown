use ratatui::{
    layout::{Rect, Alignment},
    style::Style,
    text::Text,
    widgets::{Block, Borders, BorderType, Paragraph, Wrap},
    Frame,
};

use crate::ui::app::{App, DownloadStatus};
use crate::ui::styles::{THEME_SECONDARY, THEME_PRIMARY, THEME_WARNING, THEME_SUCCESS, THEME_ERROR, THEME_TEXT};

pub fn render_download_tab(frame: &mut Frame, app: &App, area: Rect) {
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