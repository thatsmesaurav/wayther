use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub name: String,
    pub country: String,
    pub lat: f64,
    pub lon: f64,
}

impl Location {
    pub fn display_name(&self) -> String {
        format!("{}, {}", self.name, self.country)
    }
}
