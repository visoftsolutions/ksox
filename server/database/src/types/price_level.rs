use serde::{Deserialize, Serialize};

use super::Volume;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PriceLevelOption {
    pub price: Option<f64>,
    pub volume: Option<Volume>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

impl TryFrom<PriceLevelOption> for PriceLevel {
    type Error = sqlx::Error;
    fn try_from(value: PriceLevelOption) -> Result<Self, Self::Error> {
        value
            .price
            .and_then(|price| value.volume.map(|volume| Self { price, volume }))
            .ok_or(sqlx::Error::RowNotFound)
    }
}
