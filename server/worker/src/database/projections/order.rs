use chrono::{DateTime, Utc};
use fraction::Fraction;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Order {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub last_modification_at: DateTime<Utc>,
    pub user_id: Uuid,
    pub is_active: bool,
    pub quote_asset_id: Uuid,
    pub base_asset_id: Uuid,
    pub price: Fraction,
    pub quote_asset_volume: Fraction,
    pub quote_asset_volume_left: Fraction,
    pub maker_fee: Fraction,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PriceLevelOption {
    pub price: Option<f64>,
    pub volume: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PriceLevel {
    pub price: f64,
    pub volume: f64,
}

impl From<(f64, f64)> for PriceLevel {
    fn from(value: (f64, f64)) -> Self {
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
