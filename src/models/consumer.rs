use serde::{Serialize, Deserialize};
use super::ingestion::Ingestion;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConsumerWithIngestions {
    pub consumer_name: String,
    pub ingestion_elements: Vec<Ingestion>,
}