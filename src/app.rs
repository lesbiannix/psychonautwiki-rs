use ratatui::{prelude::*, widgets::*};
use crossterm::event::{self, Event, KeyCode};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppScreen {
    Home,
    Journal,
    SubstanceSearch,
}

pub struct App {
    pub screen: AppScreen,
    pub running: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            screen: AppScreen::Home,
            running: true,
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

pub fn draw_ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let block = Block::default().borders(Borders::ALL).title("PsychonautWiki Journal");
    let text = match app.screen {
        AppScreen::Home => "Home Screen",
        AppScreen::Journal => "Journal Screen",
        AppScreen::SubstanceSearch => "Substance Search Screen",
    };
    let paragraph = Paragraph::new(text).block(block);
    f.render_widget(paragraph, f.size());
}
