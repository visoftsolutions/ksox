use chrono::{DateTime, Utc};
use fraction::Fraction;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(sqlx::Type, Debug, Serialize, Deserialize, Clone, PartialEq)]
#[sqlx(type_name = "candlestick_type", rename_all = "lowercase")]
pub enum CandlestickType {
    Interval,
    Tick,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Candlestick {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub last_modification_at: DateTime<Utc>,
    pub quote_asset_id: Uuid,
    pub base_asset_id: Uuid,
    pub kind: CandlestickType,
    pub topen: DateTime<Utc>,
    pub tclose: DateTime<Utc>,
    pub open: Fraction,
    pub high: Fraction,
    pub low: Fraction,
    pub close: Fraction,
    pub span: i64,
    pub taker_quote_volume: Fraction,
    pub taker_base_volume: Fraction,
    pub maker_quote_volume: Fraction,
    pub maker_base_volume: Fraction,
}
