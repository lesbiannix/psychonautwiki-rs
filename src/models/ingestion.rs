use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ingestion {
    pub substance: String,
    pub dose: f64,
    pub units: String,
    pub route: Option<String>,
    pub time: DateTime<Utc>,
    pub notes: Option<String>,
}