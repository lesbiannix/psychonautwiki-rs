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
    app.update_filtered_substances();

    loop {
        terminal.draw(|f| draw_ui(f, &app))?;
        if event::poll(std::time::Duration::from_millis(100))? {
            if let event::Event::Key(key) = event::read()? {
                match key.code {
                    event::KeyCode::Char('q') => break,
                    event::KeyCode::Right | event::KeyCode::Tab => app.next_screen(),
                    event::KeyCode::Left => app.prev_screen(),
                    event::KeyCode::Down => {
                        match app.screen {
                            AppScreen::Journal => {
                                if app.selected_journal_index + 1 < app.journal_entries.len() {
                                    app.selected_journal_index += 1;
                                }
                            }
                            AppScreen::SubstanceSearch => {
                                if app.selected_substance_index + 1 < app.filtered_substances.len() {
                                    app.selected_substance_index += 1;
                                }
                            }
                            _ => {}
                        }
                    },
                    event::KeyCode::Up => {
                        match app.screen {
                            AppScreen::Journal => {
                                if app.selected_journal_index > 0 {
                                    app.selected_journal_index -= 1;
                                }
                            }
                            AppScreen::SubstanceSearch => {
                                if app.selected_substance_index > 0 {
                                    app.selected_substance_index -= 1;
                                }
                            }
                            _ => {}
                        }
                    },
                    event::KeyCode::Char(c) => {
                        if let AppScreen::SubstanceSearch = app.screen {
                            app.substance_search_query.push(c);
                            app.update_filtered_substances();
                        }
                    },
                    event::KeyCode::Backspace => {
                        if let AppScreen::SubstanceSearch = app.screen {
                            app.substance_search_query.pop();
                            app.update_filtered_substances();
                        }
                    },
                    event::KeyCode::Esc => {
                        if let AppScreen::SubstanceSearch = app.screen {
                            app.substance_search_query.clear();
                            app.update_filtered_substances();
                        }
                    },
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

