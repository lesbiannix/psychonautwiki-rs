use psychonaut_rs::models::experience::Experience;
use psychonaut_rs::models::substance::Substance;
use chrono::Utc;

#[test]
fn test_experience_serialization() {
    let exp = Experience {
        id: "test".to_string(),
        is_favorite: true,
        title: "Test Experience".to_string(),
        first_ingestion_time: Utc::now(),
        notes: "Test notes".to_string(),
        location_name: "Test location".to_string(),
        is_current_experience: false,
        ingestion_elements: vec![],
        cumulative_doses: vec![],
        interactions: vec![],
        ratings: vec![],
        timed_notes: vec![],
        consumers_with_ingestions: vec![],
    };
    let json = serde_json::to_string(&exp).unwrap();
    let de: Experience = serde_json::from_str(&json).unwrap();
    assert_eq!(de.id, "test");
    assert_eq!(de.title, "Test Experience");
}

#[test]
fn test_substance_serialization() {
    let sub = Substance {
        name: "LSD".to_string(),
        common_names: vec!["Acid".to_string()],
        class: Some("Psychedelic".to_string()),
        description: Some("A hallucinogen".to_string()),
        dose_units: vec!["ug".to_string()],
        default_route: Some("Oral".to_string()),
    };
    let json = serde_json::to_string(&sub).unwrap();
    let de: Substance = serde_json::from_str(&json).unwrap();
    assert_eq!(de.name, "LSD");
    assert_eq!(de.class, Some("Psychedelic".to_string()));
}
