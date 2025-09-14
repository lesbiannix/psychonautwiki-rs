# Feature Mapping: Android App → Rust TUI

| Android Feature/Screen                | Rust TUI Module/Struct Suggestion         | Notes                                      |
|---------------------------------------|-------------------------------------------|--------------------------------------------|
| Journal Home / Experience List        | `mod journal_list`, struct `Experience`   | List of journal entries                    |
| Experience Detail                     | `mod experience_detail`, struct `ExperienceDetail` | Shows all info for one experience  |
| Add/Edit Experience                   | `mod experience_edit`, struct `ExperienceForm`    | Form for new/edit experience        |
| Ingestion Entry (Add/Edit)            | `mod ingestion`, struct `Ingestion`       | Add/edit ingestion events                   |
| Dosage & Duration Tracking            | `mod dosage`, struct `CumulativeDose`     | Calculations, charts, timelines             |
| Substance Search & Database           | `mod substance_search`, struct `Substance`| Search, view, and select substances         |
| Timed Notes                           | `mod timed_notes`, struct `TimedNote`     | Time-stamped notes per experience           |
| Ratings (Shulgin, etc.)               | `mod ratings`, struct `Rating`            | Experience ratings                          |
| Interactions & Explanations           | `mod interactions`, struct `Interaction`  | Substance interaction info                  |
| Settings / Data Export                | `mod settings`, struct `Settings`         | App settings, data export/import            |
| Navigation (Tabs, Menus)              | `mod navigation`, struct `AppState`       | TUI navigation, keyboard shortcuts          |
| Data Persistence                      | `mod persistence`, struct `Database`      | JSON/SQLite, load/save data                 |
