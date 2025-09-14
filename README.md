# Psychonaut-RS

A terminal-based journal and research tool for tracking psychedelic experiences, substances, and personal insights. Built in Rust with a TUI (Text User Interface) using ratatui and crossterm.

## Features

- 📓 **Journal**: Log, view, favorite, and delete psychedelic experiences.
- 🔍 **Substance Search**: Search and browse substances with instant filtering.
- ✍️ **Experience Logging**: Multi-field form for new experiences.
- ⌨️ **Keyboard Navigation**: Fast, intuitive navigation and shortcuts.
- 🟡 **Status Bar**: See feedback for actions (save, delete, etc).

## Usage

- `nix build` or `cargo build --release` to build.
- Run with `./result/bin/psychonaut-rs` or `cargo run`.
- Keyboard shortcuts:
  - `Tab`/`→`/`←`: Switch screens
  - `↑`/`↓`: Navigate lists
  - `n`: New entry
  - `f`: Favorite/unfavorite
  - `d`: Delete entry
  - `Enter`: Save/Select
  - `Esc`: Cancel/clear
  - `q`: Quit

## Development

- Rust 2021, Nix/flake for reproducible builds
- Main code in `src/app.rs` and `src/main.rs`
- Models in `src/models/`
- Persistence in `src/persistence.rs`

## Roadmap

- [x] Core TUI navigation and forms
- [x] Journal and substance search
- [x] Status bar feedback
- [ ] Persistence (save/load to disk)
- [ ] More tests and docs
- [ ] UI polish and accessibility

---

*Inspired by the PsychonautWiki project and the need for structured, private journaling of altered states.*
