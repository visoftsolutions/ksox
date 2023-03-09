use serde::{Deserialize, Serialize};

use super::Volume;

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceLevel {
    pub price: f64,
    pub volume: Volume,
}

impl From<(f64, Volume)> for PriceLevel {
    fn from(value: (f64, Volume)) -> Self {
        Self {
            price: value.0,
            volume: value.1,
        }
    }
}
