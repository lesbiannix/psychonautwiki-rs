mod models;
mod app;
mod persistence;

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
                     event::KeyCode::Right | event::KeyCode::Tab => {
                         app.next_screen();
                         app.status_message = format!("Screen: {:?}", app.screen);
                     },
                     event::KeyCode::Left => {
                         app.prev_screen();
                         app.status_message = format!("Screen: {:?}", app.screen);
                     },
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
                                    form.field_index = (form.field_index + 1) % 4;
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
                                    if form.field_index == 0 {
                                        form.field_index = 3;
                                    } else {
                                        form.field_index -= 1;
                                    }
                                }
                            }
                            _ => {}
                        }
                    },
                    event::KeyCode::BackTab => {
                        if let AppScreen::ExperienceLogging = app.screen {
                            if let Some(form) = app.experience_form.as_mut() {
                                if form.field_index == 0 {
                                    form.field_index = 3;
                                } else {
                                    form.field_index -= 1;
                                }
                            }
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
                                 app.status_message = "Logging new experience...".to_string();
                                }
                            },
                            'f' => {
                                if let AppScreen::Journal = app.screen {
                                 if let Some(entry) = app.journal_entries.get_mut(app.selected_journal_index) {
                                     entry.is_favorite = !entry.is_favorite;
                                     app.status_message = if entry.is_favorite { "Marked as favorite.".to_string() } else { "Unfavorited.".to_string() };
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
                                     app.status_message = "Entry deleted.".to_string();
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
                    event::KeyCode::Enter => {
                        if let AppScreen::ExperienceLogging = app.screen {
                            if let Some(form) = app.experience_form.take() {
                                use chrono::Utc;
                                use crate::models::experience::Experience;
                                app.journal_entries.push(Experience {
                                    id: (app.journal_entries.len() + 1).to_string(),
                                    is_favorite: false,
                                    title: form.title,
                                    first_ingestion_time: Utc::now(),
                                    notes: form.notes,
                                    location_name: String::from(""),
                                    is_current_experience: false,
                                    ingestion_elements: vec![],
                                    cumulative_doses: vec![],
                                    interactions: vec![],
                                    ratings: vec![],
                                    timed_notes: vec![],
                                    consumers_with_ingestions: vec![],
                                });
                             app.screen = AppScreen::Journal;
                             app.status_message = "Experience saved!".to_string();
                         }
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
                                 app.status_message = "Search cleared.".to_string();
                            }
                            AppScreen::ExperienceLogging => {
                                 app.experience_form = None;
                                 app.screen = AppScreen::Journal;
                                 app.status_message = "Logging canceled.".to_string();
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

