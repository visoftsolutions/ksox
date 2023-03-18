use std::cmp::{max, min};

use chrono::Duration;
use database::{
    managers::spot::{orders::OrdersManager, trades::TradesManager},
    projections::spot::candlestick::{Candlestick, CandlestickData},
    sqlx::{self, types::Uuid, PgPool},
    types::{fraction::FractionError, CandlestickType, Fraction},
};
use thiserror::Error;
pub struct OhlcvEngine {
    pub trades_manager: TradesManager,
    pub orders_manager: OrdersManager,
}

pub struct Ohlcv {
    pub candlestick: Candlestick,
    pub merge_function: fn(&mut Candlestick, data: CandlestickData) -> Result<(), OhlcvEngineError>,
}

impl Ohlcv {
    pub fn new(candlestick: Candlestick) -> Self {
        Self {
            merge_function: match candlestick.kind {
                CandlestickType::Interval => OhlcvEngine::merge_interval,
                CandlestickType::Tick => OhlcvEngine::merge_tick,
            },
            candlestick,
        }
    }

    pub async fn merge(&mut self, data: CandlestickData) -> Result<(), OhlcvEngineError> {
        (self.merge_function)(&mut self.candlestick, data)
    }
}

impl OhlcvEngine {
    pub fn new(database: PgPool) -> Self {
        Self {
            trades_manager: TradesManager::new(database.clone()),
            orders_manager: OrdersManager::new(database),
        }
    }

    pub fn subscribe() {}

    pub fn get() {}

    pub fn merge_interval(
        candlestick: &mut Candlestick,
        data: CandlestickData,
    ) -> Result<(), OhlcvEngineError> {
        if candlestick.tclose > data.time {
            return Err(OhlcvEngineError::InvalidTime);
        }

        let price: Fraction = (
            data.taker_base_volume.to_owned(),
            data.taker_quote_volume.to_owned(),
        )
            .try_into()?;

        if candlestick.topen + Duration::microseconds(candlestick.span) < data.time {
            // data fits in interval
            candlestick.tclose = data.time;
            candlestick.high = max(candlestick.high.clone(), price.clone());
            candlestick.low = min(candlestick.low.clone(), price.clone());
            candlestick.close = price;
            candlestick.taker_quote_volume += data.taker_quote_volume;
            candlestick.taker_base_volume += data.taker_base_volume;
            candlestick.maker_quote_volume += data.maker_quote_volume;
            candlestick.maker_base_volume += data.maker_base_volume;
        } else {
            // make new candle
            candlestick.id = Uuid::new_v4();
            candlestick.topen = data.time;
            candlestick.tclose = data.time;
            candlestick.open = price.clone();
            candlestick.high = price.clone();
            candlestick.low = price.clone();
            candlestick.close = price;
            candlestick.taker_quote_volume = data.taker_quote_volume;
            candlestick.taker_base_volume = data.taker_base_volume;
            candlestick.maker_quote_volume = data.maker_quote_volume;
            candlestick.maker_base_volume = data.maker_base_volume;
        }

        Ok(())
    }

    pub fn merge_tick(
        _candlestick: &mut Candlestick,
        _data: CandlestickData,
    ) -> Result<(), OhlcvEngineError> {
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum OhlcvEngineError {
    #[error(transparent)]
    FractionError(#[from] FractionError),

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    #[error("Candlestick time invalid")]
    InvalidTime,
}
