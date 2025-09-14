use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Substance {
    pub name: String,
    pub common_names: Vec<String>,
    pub class: Option<String>,
    pub description: Option<String>,
    pub dose_units: Vec<String>,
    pub default_route: Option<String>,
    pub category: Option<String>,
}