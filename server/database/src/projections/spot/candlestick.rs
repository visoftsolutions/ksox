use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

use super::trade::Trade;
use crate::types::{fraction::FractionError, CandlestickType, Fraction, Volume};

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
    pub taker_quote_volume: Volume,
    pub taker_base_volume: Volume,
    pub maker_quote_volume: Volume,
    pub maker_base_volume: Volume,
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
            taker_base_volume: data.taker_base_volume,
            maker_quote_volume: data.maker_quote_volume,
            maker_base_volume: data.maker_base_volume,
        }
    }
}

pub struct CandlestickData {
    pub created_at: DateTime<Utc>,
    pub last_modification_at: DateTime<Utc>,
    pub quote_asset_id: Uuid,
    pub base_asset_id: Uuid,
    pub price: Fraction,
    pub time: DateTime<Utc>,
    pub taker_quote_volume: Volume,
    pub taker_base_volume: Volume,
    pub maker_quote_volume: Volume,
    pub maker_base_volume: Volume,
}

impl TryFrom<Trade> for CandlestickData {
    type Error = FractionError;
    fn try_from(value: Trade) -> Result<Self, Self::Error> {
        Ok(Self {
            created_at: value.created_at,
            last_modification_at: value.created_at,
            quote_asset_id: value.quote_asset_id,
            base_asset_id: value.base_asset_id,
            price: (
                value.taker_base_volume.clone(),
                value.taker_quote_volume.clone(),
            )
                .try_into()?,
            time: value.created_at,
            taker_quote_volume: value.taker_quote_volume,
            taker_base_volume: value.taker_base_volume,
            maker_quote_volume: value.maker_quote_volume,
            maker_base_volume: value.maker_base_volume,
        })
    }
}
