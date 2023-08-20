use chrono::{DateTime, Duration, Utc};
use fraction::Fraction;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::trade::Trade;

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
    pub maker_quote_volume: Fraction,
}

impl Candlestick {
    pub fn from_data(
        data: CandlestickData,
        kind: CandlestickType,
        reference_point: DateTime<Utc>,
        span: i64,
    ) -> Self {
        Candlestick {
            id: Uuid::new_v4(),
            created_at: data.created_at,
            last_modification_at: data.last_modification_at,
            quote_asset_id: data.quote_asset_id,
            base_asset_id: data.base_asset_id,
            kind,
            topen: data.time
                - Duration::microseconds(
                    (data.time.timestamp_micros() - reference_point.timestamp_micros())
                        .saturating_abs()
                        % span,
                ),
            tclose: data.time
                - Duration::microseconds(
                    (data.time.timestamp_micros() - reference_point.timestamp_micros())
                        .saturating_abs()
                        % span,
                )
                + Duration::microseconds(span),
            open: data.price.clone(),
            high: data.price.clone(),
            low: data.price.clone(),
            close: data.price,
            span,
            taker_quote_volume: data.taker_quote_volume,
            maker_quote_volume: data.maker_quote_volume,
        }
    }
}

pub struct CandlestickData {
    pub created_at: DateTime<Utc>,
    pub last_modification_at: DateTime<Utc>,
    pub quote_asset_id: Uuid,
    pub base_asset_id: Uuid,
    pub time: DateTime<Utc>,
    pub price: Fraction,
    pub taker_quote_volume: Fraction,
    pub maker_quote_volume: Fraction,
}

impl From<Trade> for CandlestickData {
    fn from(value: Trade) -> Self {
        Self {
            created_at: value.created_at,
            last_modification_at: value.created_at,
            quote_asset_id: value.quote_asset_id,
            base_asset_id: value.base_asset_id,
            time: value.created_at,
            price: value.price,
            taker_quote_volume: value.taker_quote_volume,
            maker_quote_volume: value.maker_quote_volume,
        }
    }
}
