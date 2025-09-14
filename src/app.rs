use ratatui::{prelude::*, widgets::*};
use crossterm::event::{self, Event, KeyCode};
mod persistence;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppScreen {
    Home,
    Journal,
    SubstanceSearch,
    ExperienceLogging,
}

use crate::models::experience::Experience;
use ratatui::widgets::ListState;

use crate::models::substance::Substance;

pub struct ExperienceForm {
    pub title: String,
    pub date: String,
    pub substance: String,
    pub notes: String,
    pub field_index: usize, // 0=title, 1=date, 2=substance, 3=notes
}

pub struct App {
    pub screen: AppScreen,
    pub running: bool,
    pub journal_entries: Vec<Experience>,
    pub selected_journal_index: usize,
    pub substances: Vec<Substance>,
    pub substance_search_query: String,
    pub filtered_substances: Vec<Substance>,
    pub selected_substance_index: usize,
    pub experience_form: Option<ExperienceForm>,
}

impl App {
    pub fn update_filtered_substances(&mut self) {
        let query = self.substance_search_query.to_lowercase();
        self.filtered_substances = if query.is_empty() {
            self.substances.clone()
        } else {
            self.substances
                .iter()
                .filter(|s| s.name.to_lowercase().contains(&query)
                    || s.common_names.iter().any(|n| n.to_lowercase().contains(&query)))
                .cloned()
                .collect()
        };
        if self.selected_substance_index >= self.filtered_substances.len() {
            self.selected_substance_index = 0;
        }
    }
}



impl ExperienceForm {
    pub fn new() -> Self {
        Self {
            title: String::new(),
            date: String::new(),
            substance: String::new(),
            notes: String::new(),
            field_index: 0,
        }
    }
}

impl App {
    pub fn new() -> Self {
        use crate::persistence::{load_journal_entries, load_substances};
        let mut journal_entries = load_journal_entries();
        if journal_entries.is_empty() {
            use chrono::Utc;
            use crate::models::experience::Experience;
            journal_entries = vec![
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
            ];
        }
        let mut substances = load_substances();
        if substances.is_empty() {
            substances = vec![
                Substance {
                    name: "LSD".to_string(),
                    common_names: vec!["Acid".to_string()],
                    class: Some("Psychedelic".to_string()),
                    description: Some("A powerful hallucinogenic substance.".to_string()),
                    dose_units: vec!["ug".to_string()],
                    default_route: Some("Oral".to_string()),
                },
                Substance {
                    name: "Psilocybin".to_string(),
                    common_names: vec!["Magic Mushrooms".to_string()],
                    class: Some("Psychedelic".to_string()),
                    description: Some("The active compound in magic mushrooms.".to_string()),
                    dose_units: vec!["mg".to_string()],
                    default_route: Some("Oral".to_string()),
                },
                Substance {
                    name: "MDMA".to_string(),
                    common_names: vec!["Ecstasy".to_string(), "Molly".to_string()],
                    class: Some("Empathogen".to_string()),
                    description: Some("A popular entactogen with stimulant properties.".to_string()),
                    dose_units: vec!["mg".to_string()],
                    default_route: Some("Oral".to_string()),
                },
            ];
        }
        Self {
            screen: AppScreen::Home,
            running: true,
            journal_entries,
            selected_journal_index: 0,
            substances,
            substance_search_query: String::new(),
            filtered_substances: vec![],
            selected_substance_index: 0,
            experience_form: None,
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
            // Split horizontally: left (list), right (details)
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([
                    Constraint::Percentage(35),
                    Constraint::Percentage(65),
                ].as_ref())
                .split(f.size());

            // Journal entry list (left)
            let items: Vec<ListItem> = app.journal_entries.iter().enumerate().map(|(i, e)| {
                let mut title = e.title.clone();
                if e.is_favorite {
                    title = format!("★ {}", title);
                }
                if e.is_current_experience {
                    title = format!("● {}", title);
                }
                ListItem::new(title)
            }).collect();
            let mut state = ListState::default();
            state.select(if app.journal_entries.is_empty() { None } else { Some(app.selected_journal_index) });
            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("Journal Entries (↑↓ to navigate, n: new, f: favorite, d: delete)"))
                .highlight_style(Style::default().bg(Color::Blue).fg(Color::White))
                .highlight_symbol("→ ");
            f.render_stateful_widget(list, chunks[0], &mut state);

            // Details pane (right)
            let details = if let Some(entry) = app.journal_entries.get(app.selected_journal_index) {
                format!(
                    "Title: {}\nDate: {}\nLocation: {}\nFavorite: {}\n\nNotes:\n{}",
                    entry.title,
                    entry.first_ingestion_time.format("%Y-%m-%d %H:%M").to_string(),
                    entry.location_name,
                    if entry.is_favorite { "Yes" } else { "No" },
                    entry.notes
                )
            } else {
                "No journal entry selected.".to_string()
            };
            let details_paragraph = Paragraph::new(details)
                .block(Block::default().borders(Borders::ALL).title("Entry Details"));
            f.render_widget(details_paragraph, chunks[1]);

            // Help bar (bottom)
            let help = Paragraph::new("Tab/→: Next screen | ←: Prev screen | n: New | f: Favorite | d: Delete | q: Quit")
                .style(Style::default().fg(Color::DarkGray))
                .block(Block::default().borders(Borders::ALL).title("Help"));
            let help_area = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(0),
                    Constraint::Length(2),
                ].as_ref())
                .split(f.size());
            f.render_widget(help, help_area[1]);
        }
        AppScreen::SubstanceSearch => {
            // Layout: input (top), list (middle), details (bottom), help (footer)
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Length(3),   // input
                    Constraint::Min(5),      // list
                    Constraint::Length(7),   // details
                    Constraint::Length(2),   // help
                ].as_ref())
                .split(f.size());

            // Input box for search query
            let input = Paragraph::new(app.substance_search_query.as_str())
                .block(Block::default().borders(Borders::ALL).title("Search Substance (type to filter, Esc to clear)"));
            f.render_widget(input, chunks[0]);

            // Filtered substance list
            let items: Vec<ListItem> = app.filtered_substances.iter().map(|s| {
                ListItem::new(s.name.clone())
            }).collect();
            let mut state = ListState::default();
            state.select(if app.filtered_substances.is_empty() { None } else { Some(app.selected_substance_index) });
            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("Results (↑↓ to navigate, Enter to select)"))
                .highlight_style(Style::default().bg(Color::Blue).fg(Color::White))
                .highlight_symbol("→ ");
            f.render_stateful_widget(list, chunks[1], &mut state);

            // Substance details
            let details = app.filtered_substances.get(app.selected_substance_index)
                .map(|s| format!("{}\nClass: {}\n{}",
                    s.name,
                    s.class.clone().unwrap_or_default(),
                    s.description.clone().unwrap_or_default()
                ))
                .unwrap_or_else(|| "No substance selected".to_string());
            let details_paragraph = Paragraph::new(details)
                .block(Block::default().borders(Borders::ALL).title("Details"));
            f.render_widget(details_paragraph, chunks[2]);

            // Help bar (footer)
            let help = Paragraph::new("Tab/→: Next screen | ←: Prev screen | ↑↓: Navigate | Enter: Select | Esc: Clear | q: Quit")
                .style(Style::default().fg(Color::DarkGray))
                .block(Block::default().borders(Borders::ALL).title("Help"));
            f.render_widget(help, chunks[3]);
        }
        AppScreen::ExperienceLogging => {
            let form = app.experience_form.as_ref().unwrap();
            let fields = [
                ("Title", &form.title),
                ("Date", &form.date),
                ("Substance", &form.substance),
                ("Notes", &form.notes),
            ];
            let mut items: Vec<ListItem> = fields.iter().enumerate().map(|(i, (label, value))| {
                let prefix = if i == form.field_index { "→ " } else { "  " };
                ListItem::new(format!("{}{}: {}", prefix, label, value))
            }).collect();
            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("Log New Experience (Tab to move, Enter to save, Esc to cancel)"))
                .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));
            f.render_widget(list, f.size());
        }
            // Input box for search query
            let input = Paragraph::new(app.substance_search_query.as_str())
                .block(Block::default().borders(Borders::ALL).title("Search Substance"));
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(5),
                    Constraint::Length(7),
                ].as_ref())
                .split(f.size());
            f.render_widget(input, chunks[0]);

            // Filtered substance list
            let items: Vec<ListItem> = app.filtered_substances.iter().map(|s| {
                ListItem::new(s.name.clone())
            }).collect();
            let mut state = ListState::default();
            state.select(Some(app.selected_substance_index));
            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("Results"))
                .highlight_style(Style::default().bg(Color::Blue).fg(Color::White))
                .highlight_symbol("→ ");
            f.render_stateful_widget(list, chunks[1], &mut state);

            // Substance details
            let details = app.filtered_substances.get(app.selected_substance_index)
                .map(|s| format!("{}\nClass: {}\n{}",
                    s.name,
                    s.class.clone().unwrap_or_default(),
                    s.description.clone().unwrap_or_default()
                ))
                .unwrap_or_else(|| "No substance selected".to_string());
            let details_paragraph = Paragraph::new(details)
                .block(Block::default().borders(Borders::ALL).title("Details"));
            f.render_widget(details_paragraph, chunks[2]);
        }
    }
}

