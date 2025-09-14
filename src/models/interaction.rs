use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Interaction {
    pub substances: Vec<String>,
    pub description: String,
    pub severity: Option<String>,
}