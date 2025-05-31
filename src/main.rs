use std::{io, path::PathBuf};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};

use crate::{
    ui::{render, app::App},
    core::Config,
    downloader::InstagramDownloader,
};

mod core;
mod downloader;
mod ui;

fn main() -> io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let config = Config::new(PathBuf::from("downloads"));
    let downloader = InstagramDownloader::new(config).expect("Failed to create downloader");
    let app = App::new(downloader);

    // Start the main loop
    let res = run_app(&mut terminal, app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| render(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::Char('Q') => {
                    if app.focused_area == ui::app::FocusedArea::ExitButton {
                        return Ok(());
                    }
                },
                KeyCode::Enter => {
                    if app.focused_area == ui::app::FocusedArea::ExitButton {
                        return Ok(());
                    }
                    app.submit_url();
                },
                KeyCode::Char('i') => app.enter_edit_mode(),
                KeyCode::Esc => app.exit_edit_mode(),
                KeyCode::Tab => app.toggle_tab(),
                KeyCode::Char(c) => {
                    if app.input_mode == ui::app::InputMode::Editing {
                        app.input.push(c);
                    }
                },
                KeyCode::Backspace => {
                    if app.input_mode == ui::app::InputMode::Editing {
                        app.input.pop();
                    }
                },
                _ => {}
            }
        } else if let Event::Mouse(mouse_event) = event::read()? {
            use crossterm::event::{MouseButton, MouseEventKind};
            
            if let MouseEventKind::Down(MouseButton::Left) = mouse_event.kind {
                // Get the terminal size
                let size = terminal.size()?;
                
                // Check if click is in the exit button area (top right)
                if mouse_event.row == 1 && mouse_event.column >= size.width - 9 {
                    app.handle_mouse_click(mouse_event.column, mouse_event.row, ui::app::FocusedArea::ExitButton);
                    return Ok(());
                }
            }
        }
    }
} 