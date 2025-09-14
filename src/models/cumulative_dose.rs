use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CumulativeDose {
    pub substance_name: String,
    pub cumulative_route_and_dose: Vec<CumulativeRouteAndDose>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CumulativeRouteAndDose {
    pub cumulative_dose: f64,
    pub units: String,
    pub is_estimate: bool,
    pub cumulative_dose_standard_deviation: Option<f64>,
    pub num_dots: Option<u32>,
    pub route: Option<String>,
    pub has_more_than_one_ingestion: bool,
}