use std::cmp::{max, min};

use chrono::Duration;
use database::{
    managers::spot::{orders::OrdersManager, trades::TradesManager},
    projections::spot::{candlestick::{Candlestick, CandlestickData}, trade::Trade},
    sqlx::{self, types::Uuid, PgPool},
    types::{fraction::FractionError, CandlestickType, Fraction}, traits::TableManager,
};
use thiserror::Error;
pub struct OhlcvEngine {
    pub trades_manager: TradesManager,
    pub orders_manager: OrdersManager,
}

pub enum MergeResult {
    Update(Candlestick),
    UpdateOldNew((Candlestick, Candlestick)),
}
pub struct Ohlcv {}

impl Ohlcv {
    pub fn merge(candlestick: Candlestick, data: CandlestickData) -> Result<MergeResult, OhlcvError> {
        match candlestick.kind {
            CandlestickType::Interval => {
                Ohlcv::merge_interval(candlestick, data)
            },
            CandlestickType::Tick => {
                Ohlcv::merge_tick(candlestick, data)
            }
        }
    }

    pub fn merge_interval(
        mut candlestick: Candlestick,
        data: CandlestickData,
    ) -> Result<MergeResult, OhlcvError> {
        if candlestick.tclose > data.time ||
        candlestick.quote_asset_id != data.quote_asset_id ||
        candlestick.base_asset_id != data.base_asset_id {
            return Err(OhlcvError::InvalidData);
        }

        let price: Fraction = (
            data.taker_base_volume.to_owned(),
            data.taker_quote_volume.to_owned(),
        )
            .try_into()?;

        return if candlestick.topen + Duration::microseconds(candlestick.span) < data.time {
            // data fits in interval
            candlestick.tclose = data.time;
            candlestick.high = max(candlestick.high.clone(), price.clone());
            candlestick.low = min(candlestick.low.clone(), price.clone());
            candlestick.close = price;
            candlestick.taker_quote_volume += data.taker_quote_volume;
            candlestick.taker_base_volume += data.taker_base_volume;
            candlestick.maker_quote_volume += data.maker_quote_volume;
            candlestick.maker_base_volume += data.maker_base_volume;

            Ok(MergeResult::Update(candlestick))
        } else {
            // make new candle, finish old one
            candlestick.tclose = data.time;

            let candlestick_new = Candlestick {
                id: Uuid::new_v4(),
                quote_asset_id: candlestick.quote_asset_id,
                base_asset_id: candlestick.base_asset_id,
                kind: candlestick.kind.clone(),
                topen: data.time,
                tclose: data.time,
                open: price.clone(),
                high: price.clone(),
                low: price.clone(),
                close: price,
                span: candlestick.span.clone(),
                taker_quote_volume: data.taker_quote_volume,
                taker_base_volume: data.taker_base_volume,
                maker_quote_volume: data.maker_quote_volume,
                maker_base_volume: data.maker_base_volume,
            };
            Ok(MergeResult::UpdateOldNew((candlestick, candlestick_new)))
        };
    }

    pub fn merge_tick(
        _candlestick: Candlestick,
        _data: CandlestickData,
    ) -> Result<MergeResult, OhlcvError> {
        todo!()
    }
}

impl OhlcvEngine {
    pub fn new(database: PgPool) -> Self {
        Self {
            trades_manager: TradesManager::new(database.clone()),
            orders_manager: OrdersManager::new(database),
        }
    }

    pub async fn merge(&self, candlestick: Candlestick, trade:Trade) -> Result<MergeResult, OhlcvEngineError> {
        let order = self.orders_manager.get_by_id(trade.order_id).await?;
        Ok(Ohlcv::merge(candlestick, (trade, order).into())?)
    }

    pub fn subscribe() {}

    pub fn get() {}
}

#[derive(Error, Debug)]
pub enum OhlcvError {
    #[error(transparent)]
    FractionError(#[from] FractionError),

    #[error("CandlestickData invalid")]
    InvalidData,
}

#[derive(Error, Debug)]
pub enum OhlcvEngineError {
    #[error(transparent)]
    OhlcvError(#[from] OhlcvError),

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}