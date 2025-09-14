use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use crate::models::experience::Experience;
use crate::models::substance::Substance;

const JOURNAL_PATH: &str = "journal_entries.json";
const SUBSTANCES_PATH: &str = "substances.json";

pub fn load_journal_entries() -> Vec<Experience> {
    if Path::new(JOURNAL_PATH).exists() {
        let mut file = File::open(JOURNAL_PATH).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        Vec::new()
    }
}

pub fn save_journal_entries(entries: &[Experience]) {
    let data = serde_json::to_string_pretty(entries).unwrap();
    let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(JOURNAL_PATH).unwrap();
    file.write_all(data.as_bytes()).unwrap();
}

pub fn load_substances() -> Vec<Substance> {
    if Path::new(SUBSTANCES_PATH).exists() {
        let mut file = File::open(SUBSTANCES_PATH).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        Vec::new()
    }
}

pub fn save_substances(substances: &[Substance]) {
    let data = serde_json::to_string_pretty(substances).unwrap();
    let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(SUBSTANCES_PATH).unwrap();
    file.write_all(data.as_bytes()).unwrap();
}
