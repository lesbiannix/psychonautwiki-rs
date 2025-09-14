use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Rating {
    pub scale: String, // e.g. "Shulgin"
    pub value: f32,
    pub note: Option<String>,
}