use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

use crate::types::{CandlestickType, Fraction, Volume};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Candlestick {
    pub id: Uuid,
    pub quote_asset_id: Uuid,
    pub base_asset_id: Uuid,
    pub kind: CandlestickType,
    pub topen: DateTime<Utc>,
    pub tclose: DateTime<Utc>,
    pub open: Fraction,
    pub high: Fraction,
    pub low: Fraction,
    pub close: Fraction,
    pub span: Volume,
    pub taker_quote_volume: Volume,
    pub taker_base_volume: Volume,
    pub maker_quote_volume: Volume,
    pub maker_base_volume: Volume,
}
