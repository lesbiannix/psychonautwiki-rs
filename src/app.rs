use ratatui::{prelude::*, widgets::*};
use crossterm::event::{self, Event, KeyCode};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppScreen {
    Home,
    Journal,
    SubstanceSearch,
}

use crate::models::experience::Experience;
use ratatui::widgets::ListState;

pub struct App {
    pub screen: AppScreen,
    pub running: bool,
    pub journal_entries: Vec<Experience>,
    pub selected_journal_index: usize,
}


impl App {
    pub fn new() -> Self {
        use chrono::Utc;
        use crate::models::experience::Experience;
        Self {
            screen: AppScreen::Home,
            running: true,
            journal_entries: vec![
                Experience {
                    id: "1".to_string(),
                    is_favorite: false,
                    title: "First LSD Experience".to_string(),
                    first_ingestion_time: Utc::now(),
                    notes: "A profound journey.".to_string(),
                    location_name: "Home".to_string(),
                    is_current_experience: false,
                    ingestion_elements: vec![],
                    cumulative_doses: vec![],
                    interactions: vec![],
                    ratings: vec![],
                    timed_notes: vec![],
                    consumers_with_ingestions: vec![],
                },
                Experience {
                    id: "2".to_string(),
                    is_favorite: true,
                    title: "Psilocybin Adventure".to_string(),
                    first_ingestion_time: Utc::now(),
                    notes: "Lots of visuals.".to_string(),
                    location_name: "Forest".to_string(),
                    is_current_experience: false,
                    ingestion_elements: vec![],
                    cumulative_doses: vec![],
                    interactions: vec![],
                    ratings: vec![],
                    timed_notes: vec![],
                    consumers_with_ingestions: vec![],
                },
            ],
            selected_journal_index: 0,
        }
    }

    pub fn next_screen(&mut self) {
        self.screen = match self.screen {
            AppScreen::Home => AppScreen::Journal,
            AppScreen::Journal => AppScreen::SubstanceSearch,
            AppScreen::SubstanceSearch => AppScreen::Home,
        };
    }

    pub fn prev_screen(&mut self) {
        self.screen = match self.screen {
            AppScreen::Home => AppScreen::SubstanceSearch,
            AppScreen::Journal => AppScreen::Home,
            AppScreen::SubstanceSearch => AppScreen::Journal,
        };
    }
}

pub fn draw_ui(f: &mut Frame, app: &App) {
    let block = Block::default().borders(Borders::ALL).title("PsychonautWiki Journal");
    match app.screen {
        AppScreen::Home => {
            let paragraph = Paragraph::new("Home Screen").block(block);
            f.render_widget(paragraph, f.size());
        }
        AppScreen::Journal => {
            let items: Vec<ListItem> = app.journal_entries.iter().map(|e| {
                ListItem::new(e.title.clone())
            }).collect();
            let mut state = ListState::default();
            state.select(Some(app.selected_journal_index));
            let list = List::new(items)
                .block(block)
                .highlight_style(Style::default().bg(Color::Blue).fg(Color::White))
                .highlight_symbol("→ ");
            f.render_stateful_widget(list, f.size(), &mut state);
        }
        AppScreen::SubstanceSearch => {
            let paragraph = Paragraph::new("Substance Search Screen").block(block);
            f.render_widget(paragraph, f.size());
        }
    }
}

