use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

use super::ingestion::Ingestion;
use super::cumulative_dose::CumulativeDose;
use super::consumer::ConsumerWithIngestions;
use super::rating::Rating;
use super::timed_note::TimedNote;
use super::interaction::Interaction;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Experience {
    pub id: String,
    pub is_favorite: bool,
    pub title: String,
    pub first_ingestion_time: DateTime<Utc>,
    pub notes: String,
    pub location_name: String,
    pub category: Option<String>,
    pub is_current_experience: bool,
    pub ingestion_elements: Vec<Ingestion>,
    pub cumulative_doses: Vec<CumulativeDose>,
    pub interactions: Vec<Interaction>,
    pub ratings: Vec<Rating>,
    pub timed_notes: Vec<TimedNote>,
    pub consumers_with_ingestions: Vec<ConsumerWithIngestions>,
}