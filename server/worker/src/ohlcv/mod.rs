use std::{
    cmp::{max, min},
    pin::Pin,
};

use chrono::{DateTime, Duration, Utc};
use database::{
    managers::{
        candlesticks::CandlesticksManager,
        trades::{TradesManager, TradesNotificationManager},
    },
    projections::candlestick::{Candlestick, CandlestickData},
};
use futures::{FutureExt, Stream, StreamExt};
use thiserror::Error;
use uuid::Uuid;

use crate::database::{self, projections::candlestick::CandlestickType};
pub struct OhlcvEngine {
    trades_manager: TradesManager,
    trades_notification_manager: TradesNotificationManager,
    candlesticks_manager: CandlesticksManager,
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
            candlestick.maker_quote_volume += data.maker_quote_volume;
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
    pub fn new(
        trades_manager: TradesManager,
        trades_notification_manager: TradesNotificationManager,
        candlesticks_manager: CandlesticksManager,
    ) -> Self {
        Self {
            trades_manager,
            trades_notification_manager,
            candlesticks_manager,
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
            let mut trades = self.trades_notification_manager.subscribe_to_asset_pair(quote_asset_id, base_asset_id).await?
            .map(|trades| {
                let mut result = Vec::new();
                for trade in trades {
                    if trade.is_opposite(quote_asset_id, base_asset_id) {
                        result.push(trade.inverse());
                    } else {
                        result.push(trade);
                    }
                }
                result
            });

            let last_trade = self
                .trades_manager
                .get_last_for_asset_pair(quote_asset_id, base_asset_id)
                .await?;

            if let Some(last_trade) = last_trade {
                if reference_point <= last_trade.created_at {
                    let last_topen = match kind {
                        CandlestickType::Interval => {
                            last_trade.created_at - Duration::microseconds((last_trade.created_at.timestamp_micros() - reference_point.timestamp_micros()).saturating_abs() % span)
                        }
                        CandlestickType::Tick => {
                            // TODO code it
                            last_trade.created_at
                        }
                    };

                    let mut last_candle_trades = self.trades_manager.get_after_for_asset_pair(quote_asset_id, base_asset_id, last_topen)
                    .map(|trade| {
                        match trade {
                            Ok(trade) => {
                                if trade.is_opposite(quote_asset_id, base_asset_id) {
                                    Ok(trade.inverse())
                                } else {
                                    Ok(trade)
                                }
                            },
                            Err(err) => Err(err)
                        }
                    });

                    while let Some(trade) = last_candle_trades.next().await {
                        self.update(&mut candlestick, trade?.into(), kind.to_owned(),reference_point.to_owned(), span)?;
                    }
                }
            };

            if let Some(candlestick) = candlestick.to_owned() {
                yield candlestick;
            }

            while let Some(trades) = trades.next().await {
                for trade in trades {
                    yield self.update(&mut candlestick, trade.into(), kind.to_owned(),reference_point.to_owned(), span)?;
                }
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
                            let mut trades = self
                                .trades_manager
                                .get_between_for_asset_pair(
                                    quote_asset_id,
                                    base_asset_id,
                                    topen,
                                    tclose,
                                )
                                .map(|trade| match trade {
                                    Ok(trade) => {
                                        if trade.is_opposite(quote_asset_id, base_asset_id) {
                                            Ok(trade.inverse())
                                        } else {
                                            Ok(trade)
                                        }
                                    }
                                    Err(err) => Err(err),
                                });
                            while let Some(trade) = trades.next().await {
                                self.update(
                                    &mut candlestick,
                                    trade?.into(),
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
    #[error("CandlestickData invalid")]
    InvalidData,
}

#[derive(Error, Debug)]
pub enum OhlcvEngineError {
    #[error(transparent)]
    OhlcvError(#[from] OhlcvError),

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    #[error("Custom error {0}")]
    CustomError(String),
}
