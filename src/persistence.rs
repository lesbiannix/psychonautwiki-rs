use std::fs::{File, OpenOptions, rename, remove_file};
use std::io::{self, Read, Write};
use std::path::Path;
use serde_json;
use crate::models::experience::Experience;
use crate::models::substance::Substance;

#[derive(Debug)]
pub enum PersistenceError {
    Io(io::Error),
    Serde(serde_json::Error),
    AtomicSwapFailed(io::Error),
}

impl std::fmt::Display for PersistenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PersistenceError::Io(e) => write!(f, "I/O error: {}", e),
            PersistenceError::Serde(e) => write!(f, "Serialization error: {}", e),
            PersistenceError::AtomicSwapFailed(e) => write!(f, "Atomic file swap failed: {}", e),
        }
    }
}

impl std::error::Error for PersistenceError {}

impl From<io::Error> for PersistenceError {
    fn from(e: io::Error) -> Self {
        PersistenceError::Io(e)
    }
}

impl From<serde_json::Error> for PersistenceError {
    fn from(e: serde_json::Error) -> Self {
        PersistenceError::Serde(e)
    }
}

const JOURNAL_PATH: &str = "journal_entries.json";
const SUBSTANCES_PATH: &str = "substances.json";

/// Loads journal entries from the JSON file.
/// Returns Ok(Vec<Experience>) on success, or Err(PersistenceError) on failure.
pub fn load_journal_entries() -> Result<Vec<Experience>, PersistenceError> {
    if Path::new(JOURNAL_PATH).exists() {
        let mut file = File::open(JOURNAL_PATH)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        let entries = serde_json::from_str(&data)?;
        Ok(entries)
    } else {
        Ok(Vec::new())
    }
}

/// Saves journal entries to the JSON file atomically.
/// Returns Ok(()) on success, or Err(PersistenceError) on failure.
pub fn save_journal_entries(entries: &[Experience]) -> Result<(), PersistenceError> {
    let data = serde_json::to_string_pretty(entries)?;
    let tmp_path = format!("{}.tmp", JOURNAL_PATH);
    {
        let mut tmp_file = OpenOptions::new().write(true).create(true).truncate(true).open(&tmp_path)?;
        tmp_file.write_all(data.as_bytes())?;
        tmp_file.flush()?;
    }
    // Atomically replace the original file
    rename(&tmp_path, JOURNAL_PATH).map_err(PersistenceError::AtomicSwapFailed)?;
    Ok(())
}

/// Loads substances from the JSON file.
/// Returns Ok(Vec<Substance>) on success, or Err(PersistenceError) on failure.
pub fn load_substances() -> Result<Vec<Substance>, PersistenceError> {
    if Path::new(SUBSTANCES_PATH).exists() {
        let mut file = File::open(SUBSTANCES_PATH)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        let substances = serde_json::from_str(&data)?;
        Ok(substances)
    } else {
        Ok(Vec::new())
    }
}

/// Saves substances to the JSON file atomically.
/// Returns Ok(()) on success, or Err(PersistenceError) on failure.
pub fn save_substances(substances: &[Substance]) -> Result<(), PersistenceError> {
    let data = serde_json::to_string_pretty(substances)?;
    let tmp_path = format!("{}.tmp", SUBSTANCES_PATH);
    {
        let mut tmp_file = OpenOptions::new().write(true).create(true).truncate(true).open(&tmp_path)?;
        tmp_file.write_all(data.as_bytes())?;
        tmp_file.flush()?;
    }
    // Atomically replace the original file
    rename(&tmp_path, SUBSTANCES_PATH).map_err(PersistenceError::AtomicSwapFailed)?;
    Ok(())
}
