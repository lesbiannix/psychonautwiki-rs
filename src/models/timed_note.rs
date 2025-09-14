use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TimedNote {
    pub time: DateTime<Utc>,
    pub note: String,
}