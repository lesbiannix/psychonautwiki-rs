use psychonaut_rs::models::experience::Experience;
use psychonaut_rs::models::substance::Substance;
use psychonaut_rs::persistence::{save_journal_entries, load_journal_entries, save_substances, load_substances};
use chrono::Utc;
use std::fs;

#[test]
fn test_journal_persistence() {
    let exp = Experience {
        id: "persist1".to_string(),
        is_favorite: false,
        title: "Persist Test".to_string(),
        first_ingestion_time: Utc::now(),
        notes: "Persistence notes".to_string(),
        location_name: "Lab".to_string(),
        is_current_experience: false,
        ingestion_elements: vec![],
        cumulative_doses: vec![],
        interactions: vec![],
        ratings: vec![],
        timed_notes: vec![],
        consumers_with_ingestions: vec![],
    };
    save_journal_entries(&[exp.clone()]);
    let loaded = load_journal_entries();
    assert_eq!(loaded[0].id, "persist1");
    fs::remove_file("journal_entries.json").unwrap();
}

#[test]
fn test_substance_persistence() {
    let sub = Substance {
        name: "PersistSub".to_string(),
        common_names: vec!["Test".to_string()],
        class: Some("TestClass".to_string()),
        description: Some("TestDesc".to_string()),
        dose_units: vec!["mg".to_string()],
        default_route: Some("Oral".to_string()),
    };
    save_substances(&[sub.clone()]);
    let loaded = load_substances();
    assert_eq!(loaded[0].name, "PersistSub");
    fs::remove_file("substances.json").unwrap();
}
