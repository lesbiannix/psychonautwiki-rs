mod models;
mod app;

use app::{App, AppScreen, draw_ui};
use crossterm::{event, execute, terminal};
use ratatui::prelude::*;
use std::io::{self, Stdout};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Terminal setup
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, terminal::EnterAlternateScreen, event::EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    loop {
        terminal.draw(|f| draw_ui(f, &app))?;
        if event::poll(std::time::Duration::from_millis(100))? {
            if let event::Event::Key(key) = event::read()? {
                match key.code {
                    event::KeyCode::Char('q') => break,
                    event::KeyCode::Right | event::KeyCode::Tab => app.next_screen(),
                    event::KeyCode::Left => app.prev_screen(),
                    _ => {}
                }
            }
        }
        if !app.running {
            break;
        }
    }

    // Restore terminal
    terminal::disable_raw_mode()?;
    execute!(terminal.backend_mut(), terminal::LeaveAlternateScreen, event::DisableMouseCapture)?;
    Ok(())
}

