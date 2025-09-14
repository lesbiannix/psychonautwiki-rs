# Technical Decisions: Initial Rust Project Structure & Dependencies

## Project Structure Proposal

- src/
  - main.rs (entry point)
  - app.rs (main app state and navigation)
  - journal_list.rs (list of experiences)
  - experience_detail.rs (view/edit experience)
  - experience_edit.rs (form for new/edit experience)
  - ingestion.rs (add/edit ingestion events)
  - dosage.rs (cumulative dose calculations)
  - substance_search.rs (substance database/search)
  - timed_notes.rs (timed notes per experience)
  - ratings.rs (experience ratings)
  - interactions.rs (substance interactions)
  - settings.rs (settings, data export/import)
  - persistence.rs (data persistence: JSON/SQLite)
  - navigation.rs (TUI navigation, keyboard shortcuts)
  - models/
    - experience.rs
    - ingestion.rs
    - substance.rs
    - rating.rs
    - interaction.rs
    - timed_note.rs
    - user.rs

## Dependencies (Cargo.toml)

```
[dependencies]
ratatui = "0.24"
crossterm = "0.27"
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }
reqwest = { version = "0.11", features = ["json"] }
clap = { version = "4.0", features = ["derive"] }
```

## Notes
- Modular structure for maintainability and feature parity with Android app
- Async runtime (tokio) for API and file I/O
- Serde/serde_json for data serialization
- Optionally add SQLite support (e.g., rusqlite) if needed
- TUI navigation and keyboard shortcuts via ratatui/crossterm
- All session management and progress tracking in ai/
