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
                            AppScreen::ExperienceLogging => {
                                if let Some(form) = app.experience_form.as_mut() {
                                    if form.field_index < 3 {
                                        form.field_index += 1;
                                    }
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
                            AppScreen::ExperienceLogging => {
                                if let Some(form) = app.experience_form.as_mut() {
                                    if form.field_index > 0 {
                                        form.field_index -= 1;
                                    }
                                }
                            }
                            _ => {}
                        }
                    },
                     event::KeyCode::Char(c) => {
                        match c {
                            '1' => app.screen = AppScreen::Journal,
                            '2' => app.screen = AppScreen::SubstanceSearch,
                            '3' => {
                                app.screen = AppScreen::ExperienceLogging;
                                if app.experience_form.is_none() {
                                    app.experience_form = Some(app::ExperienceForm::new());
                                }
                            },
                            'n' => {
                                if let AppScreen::Journal = app.screen {
                                    app.screen = AppScreen::ExperienceLogging;
                                    app.experience_form = Some(app::ExperienceForm::new());
                                }
                            },
                            'f' => {
                                if let AppScreen::Journal = app.screen {
                                    if let Some(entry) = app.journal_entries.get_mut(app.selected_journal_index) {
                                        entry.is_favorite = !entry.is_favorite;
                                    }
                                }
                            },
                            'd' => {
                                if let AppScreen::Journal = app.screen {
                                    if app.journal_entries.len() > 0 {
                                        app.journal_entries.remove(app.selected_journal_index);
                                        if app.selected_journal_index >= app.journal_entries.len() && app.selected_journal_index > 0 {
                                            app.selected_journal_index -= 1;
                                        }
                                    }
                                }
                            },
                            'h' => app.screen = AppScreen::Home,
                            _ => match app.screen {
                                AppScreen::SubstanceSearch => {
                                    app.substance_search_query.push(c);
                                    app.update_filtered_substances();
                                }
                                AppScreen::ExperienceLogging => {
                                    if let Some(form) = app.experience_form.as_mut() {
                                        match form.field_index {
                                            0 => form.title.push(c),
                                            1 => form.date.push(c),
                                            2 => form.substance.push(c),
                                            3 => form.notes.push(c),
                                            _ => {}
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    },
                     event::KeyCode::Backspace => {
                        match app.screen {
                            AppScreen::SubstanceSearch => {
                                app.substance_search_query.pop();
                                app.update_filtered_substances();
                            }
                            AppScreen::ExperienceLogging => {
                                if let Some(form) = app.experience_form.as_mut() {
                                    match form.field_index {
                                        0 => { form.title.pop(); },
                                        1 => { form.date.pop(); },
                                        2 => { form.substance.pop(); },
                                        3 => { form.notes.pop(); },
                                        _ => {}
                                    }
                                }
                            }
                            _ => {}
                        }
                    },
                     event::KeyCode::Esc => {
                        match app.screen {
                            AppScreen::SubstanceSearch => {
                                app.substance_search_query.clear();
                                app.update_filtered_substances();
                            }
                            _ => {
                                app.screen = AppScreen::Home;
                            }
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

