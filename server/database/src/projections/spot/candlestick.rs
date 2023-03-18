use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

use super::{order::Order, trade::Trade};
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
    pub span: i64,
    pub taker_quote_volume: Volume,
    pub taker_base_volume: Volume,
    pub maker_quote_volume: Volume,
    pub maker_base_volume: Volume,
}

pub struct CandlestickData {
    pub quote_asset_id: Uuid,
    pub base_asset_id: Uuid,
    pub time: DateTime<Utc>,
    pub taker_quote_volume: Volume,
    pub taker_base_volume: Volume,
    pub maker_quote_volume: Volume,
    pub maker_base_volume: Volume,
}

impl From<(Trade, Order)> for CandlestickData {
    fn from(value: (Trade, Order)) -> Self {
        Self {
            quote_asset_id: value.1.quote_asset_id,
            base_asset_id: value.1.base_asset_id,
            time: value.0.created_at,
            taker_quote_volume: value.0.taker_quote_volume,
            taker_base_volume: value.0.taker_base_volume,
            maker_quote_volume: value.0.maker_quote_volume,
            maker_base_volume: value.0.maker_base_volume,
        }
    }
}
