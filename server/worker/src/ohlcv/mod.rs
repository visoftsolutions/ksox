use std::{
    cmp::{max, min},
    pin::Pin,
};

use chrono::{DateTime, Duration, Utc};
use database::{
    managers::spot::{
        candlesticks::CandlestickManager, orders::OrdersManager, trades::TradesManager,
    },
    projections::spot::candlestick::{Candlestick, CandlestickData},
    sqlx::{self, types::Uuid, PgPool},
    traits::TableManager,
    types::{fraction::FractionError, CandlestickType},
};
use futures::{FutureExt, Stream, StreamExt};
use thiserror::Error;
pub struct OhlcvEngine {
    pub trades_manager: TradesManager,
    pub orders_manager: OrdersManager,
    pub candlesticks_manager: CandlestickManager,
}
pub struct Ohlcv {}

impl Ohlcv {
    pub fn merge(
        candlestick: Candlestick,
        data: CandlestickData,
        reference_point: DateTime<Utc>,
        span: i64,
    ) -> Result<Candlestick, OhlcvError> {
        match candlestick.kind {
            CandlestickType::Interval => {
                Ohlcv::merge_interval(candlestick, data, reference_point, span)
            }
            CandlestickType::Tick => Ohlcv::merge_tick(candlestick, data),
        }
    }

    pub fn merge_interval(
        mut candlestick: Candlestick,
        data: CandlestickData,
        reference_point: DateTime<Utc>,
        span: i64,
    ) -> Result<Candlestick, OhlcvError> {
        if candlestick.quote_asset_id != data.quote_asset_id
            || candlestick.base_asset_id != data.base_asset_id
        {
            return Err(OhlcvError::InvalidData);
        }

        if candlestick.topen <= data.time && data.time < candlestick.tclose {
            candlestick.high = max(&candlestick.high, &data.price).to_owned();
            candlestick.low = min(&candlestick.low, &data.price).to_owned();
            candlestick.close = data.price;
            candlestick.taker_quote_volume += data.taker_quote_volume;
            candlestick.taker_base_volume += data.taker_base_volume;
            candlestick.maker_quote_volume += data.maker_quote_volume;
            candlestick.maker_base_volume += data.maker_base_volume;
            Ok(candlestick)
        } else {
            Ok(Candlestick::from_data(
                data,
                candlestick.kind,
                reference_point,
                span,
            ))
        }
    }

    pub fn merge_tick(
        _candlestick: Candlestick,
        _data: CandlestickData,
    ) -> Result<Candlestick, OhlcvError> {
        todo!()
    }
}

impl OhlcvEngine {
    pub fn new(database: PgPool) -> Self {
        Self {
            trades_manager: TradesManager::new(database.clone()),
            orders_manager: OrdersManager::new(database.clone()),
            candlesticks_manager: CandlestickManager::new(database),
        }
    }

    pub fn merge(
        &self,
        candlestick: Candlestick,
        data: CandlestickData,
        reference_point: DateTime<Utc>,
        span: i64,
    ) -> Result<Candlestick, OhlcvEngineError> {
        Ok(Ohlcv::merge(candlestick, data, reference_point, span)?)
    }

    pub fn update(
        &self,
        candlestick: &mut Option<Candlestick>,
        data: CandlestickData,
        kind: CandlestickType,
        reference_point: DateTime<Utc>,
        span: i64,
    ) -> Result<Candlestick, OhlcvEngineError> {
        Ok(match candlestick.take() {
            Some(c) => {
                let c = self.merge(c, data, reference_point.to_owned(), span)?;
                *candlestick = Some(c.clone());
                c
            }
            None => {
                let c = Candlestick::from_data(data, kind, reference_point, span);
                *candlestick = Some(c.clone());
                c
            }
        })
    }

    pub async fn subscribe(
        &self,
        quote_asset_id: Uuid,
        base_asset_id: Uuid,
        kind: CandlestickType,
        reference_point: DateTime<Utc>,
        span: i64,
    ) -> Pin<Box<dyn Stream<Item = Result<Candlestick, OhlcvEngineError>> + Send + '_>> {
        let stream = async_stream::try_stream! {
            let mut candlestick: Option<Candlestick> = None;
            let mut trades = self.trades_manager.subscribe_for_asset_pair(quote_asset_id, base_asset_id).await?;

            let last_trade = self
                .trades_manager
                .get_last_for_asset_pair(quote_asset_id, base_asset_id)
                .await?;

            if let Some(last_trade) = last_trade {
                let last_topen = match kind {
                    CandlestickType::Interval => {
                        last_trade.created_at - Duration::microseconds((last_trade.created_at.timestamp_micros() - reference_point.timestamp_micros()).saturating_abs() % span)
                    }
                    CandlestickType::Tick => {
                        // TODO code it
                        last_trade.created_at
                    }
                };

                let mut last_candle_trades = self.trades_manager.get_after_for_asset_pair(quote_asset_id, base_asset_id, last_topen);

                while let Some(trade) = last_candle_trades.next().await {
                    let data: CandlestickData = trade?.try_into()?;
                    self.update(&mut candlestick, data, kind.to_owned(),reference_point.to_owned(), span)?;
                }

            };

            if let Some(candlestick) = candlestick.to_owned() {
                yield candlestick;
            }

            while let Some(trade) = trades.next().await {
                let data: CandlestickData = trade?.try_into()?;
                yield self.update(&mut candlestick, data, kind.to_owned(),reference_point.to_owned(), span)?;
            }
        };
        Box::pin(stream)
    }

    pub async fn get(
        &self,
        quote_asset_id: Uuid,
        base_asset_id: Uuid,
        kind: CandlestickType,
        reference_point: DateTime<Utc>,
        span: i64,
    ) -> Result<Option<Candlestick>, OhlcvEngineError> {
        match kind {
            CandlestickType::Interval => {
                let topen = reference_point;
                let tclose = reference_point + Duration::microseconds(span);
                self.candlesticks_manager
                    .get_interval_for_asset_pair(quote_asset_id, base_asset_id, topen, tclose)
                    .then(|result| async move {
                        let mut candlestick = result?;
                        if candlestick.is_none() {
                            let mut trades = self.trades_manager.get_between_for_asset_pair(
                                quote_asset_id,
                                base_asset_id,
                                topen,
                                tclose,
                            );
                            while let Some(trade) = trades.next().await {
                                let data: CandlestickData = trade?.try_into()?;
                                self.update(
                                    &mut candlestick,
                                    data,
                                    kind.to_owned(),
                                    reference_point.to_owned(),
                                    span,
                                )?;
                            }
                            if let Some(candlestick) = candlestick.clone() {
                                self.candlesticks_manager.insert(candlestick).await?;
                            }
                        }
                        Ok(candlestick)
                    })
                    .await
            }
            CandlestickType::Tick => {
                // TODO code it
                Err(OhlcvEngineError::CustomError(
                    "tick chart not implemented yet".to_owned(),
                ))
            }
        }
    }
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

    #[error(transparent)]
    FractionError(#[from] FractionError),

    #[error("Custom error {0}")]
    CustomError(String),
}
