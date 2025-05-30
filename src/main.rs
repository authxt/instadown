mod ui;
mod downloader;

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use std::io;

use crate::ui::app::{App, InputMode, DownloadStatus};
use crate::ui::ui::render;
use crate::downloader::Downloader;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Instagram post URL
    #[arg(short, long)]
    url: String,

    /// Output directory (optional)
    #[arg(short, long, default_value = "downloads")]
    output: PathBuf,
}

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new();
    let downloader = match Downloader::new(app.base_path.clone()) {
        Ok(d) => d,
        Err(e) => {
            // Clean up terminal before showing error
            disable_raw_mode()?;
            execute!(
                terminal.backend_mut(),
                LeaveAlternateScreen,
                DisableMouseCapture
            )?;
            terminal.show_cursor()?;
            return Err(e);
        }
    };

    // Main loop
    loop {
        terminal.draw(|frame| render(frame, &app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match app.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('i') => app.enter_edit_mode(),
                        KeyCode::Tab => app.toggle_tab(),
                        _ => {}
                    },
                    InputMode::Editing => match key.code {
                        KeyCode::Enter => {
                            if !app.input.is_empty() {
                                app.download_status = DownloadStatus::InProgress;
                                match downloader.download(&app.input) {
                                    Ok(filename) => {
                                        app.add_download(app.input.clone(), filename);
                                        app.input.clear();
                                        app.exit_edit_mode();
                                        app.download_status = DownloadStatus::Complete;
                                    }
                                    Err(e) => {
                                        app.download_status = DownloadStatus::Error(e.to_string());
                                    }
                                }
                            }
                        }
                        KeyCode::Esc => {
                            app.exit_edit_mode();
                        }
                        KeyCode::Char(c) => {
                            app.input.push(c);
                        }
                        KeyCode::Backspace => {
                            app.input.pop();
                        }
                        _ => {}
                    }
                }
            }
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