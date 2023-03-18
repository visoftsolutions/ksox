use std::{
    cmp::{max, min},
    pin::Pin,
};

use chrono::{DateTime, Duration, Utc};
use database::{
    managers::spot::{orders::OrdersManager, trades::TradesManager},
    projections::spot::candlestick::{Candlestick, CandlestickData},
    sqlx::{self, types::Uuid, PgPool},
    traits::TableManager,
    types::{fraction::FractionError, CandlestickType},
};
use futures::{Stream, StreamExt};
use thiserror::Error;
pub struct OhlcvEngine {
    pub trades_manager: TradesManager,
    pub orders_manager: OrdersManager,
}
pub struct Ohlcv {}

impl Ohlcv {
    pub fn merge(
        candlestick: Candlestick,
        data: CandlestickData,
    ) -> Result<Candlestick, OhlcvError> {
        match candlestick.kind {
            CandlestickType::Interval => Ohlcv::merge_interval(candlestick, data),
            CandlestickType::Tick => Ohlcv::merge_tick(candlestick, data),
        }
    }

    pub fn merge_interval(
        mut candlestick: Candlestick,
        data: CandlestickData,
    ) -> Result<Candlestick, OhlcvError> {
        if candlestick.tclose > data.time
            || candlestick.quote_asset_id != data.quote_asset_id
            || candlestick.base_asset_id != data.base_asset_id
        {
            return Err(OhlcvError::InvalidData);
        }

        if candlestick.topen <= data.time && data.time < candlestick.tclose {
            candlestick.tclose = data.time;
            candlestick.high = max(&candlestick.high, &data.price).to_owned();
            candlestick.low = min(&candlestick.low, &data.price).to_owned();
            candlestick.close = data.price;
            candlestick.taker_quote_volume += data.taker_quote_volume;
            candlestick.taker_base_volume += data.taker_base_volume;
            candlestick.maker_quote_volume += data.maker_quote_volume;
            candlestick.maker_base_volume += data.maker_base_volume;
            Ok(candlestick)
        } else {
            Ok(Candlestick {
                id: Uuid::new_v4(),
                quote_asset_id: data.quote_asset_id,
                base_asset_id: data.base_asset_id,
                kind: candlestick.kind.clone(),
                topen: data.time,
                tclose: data.time + Duration::milliseconds(candlestick.span),
                open: data.price.clone(),
                high: data.price.clone(),
                low: data.price.clone(),
                close: data.price,
                span: candlestick.span,
                taker_quote_volume: data.taker_quote_volume,
                taker_base_volume: data.taker_base_volume,
                maker_quote_volume: data.maker_quote_volume,
                maker_base_volume: data.maker_base_volume,
            })
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
            orders_manager: OrdersManager::new(database),
        }
    }

    pub async fn merge(
        &self,
        candlestick: Candlestick,
        data: CandlestickData,
    ) -> Result<Candlestick, OhlcvEngineError> {
        Ok(Ohlcv::merge(candlestick, data)?)
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
            let last_trade = self
                .trades_manager
                .get_last_for_asset_pair(quote_asset_id, base_asset_id)
                .await?;

            let mut trades_stream = Box::pin(async_stream::try_stream! {
                let mut trades = self.trades_manager.subscribe_for_asset_pair(quote_asset_id, base_asset_id).await?;
                match last_trade {
                    Some(last_trade) => {
                        let last_topen = last_trade.created_at
                            - Duration::milliseconds(
                                (last_trade.created_at.timestamp_millis() - reference_point.timestamp_millis())
                                    % span,
                            );

                        let last_candle_trades = self.trades_manager.get_after_for_asset_pair(quote_asset_id, base_asset_id, last_topen);
                        let mut trades_chained = last_candle_trades.chain(trades);

                        while let Some(trade) = trades_chained.next().await {
                            yield trade;
                        }
                    },
                    None => {
                        while let Some(trade) = trades.next().await {
                            yield trade;
                        }
                    }
                }
            }.map(|a| a.and_then(|b| b)));

            while let Some(trade) = trades_stream.next().await {
                let trade = trade?;
                let order = self.orders_manager.get_by_id(trade.order_id).await?;
                let data: CandlestickData = (trade, order).try_into()?;
                yield match candlestick {
                    Some(c) => {
                        let c = self.merge(c, data).await?;
                        candlestick = Some(c.clone());
                        c
                    }
                    None => {
                        let c = Candlestick {
                            id: Uuid::new_v4(),
                            quote_asset_id: data.quote_asset_id,
                            base_asset_id: data.base_asset_id,
                            kind: kind.clone(),
                            topen: data.time,
                            tclose: data.time + Duration::milliseconds(span),
                            open: data.price.clone(),
                            high: data.price.clone(),
                            low: data.price.clone(),
                            close: data.price,
                            span,
                            taker_quote_volume: data.taker_quote_volume,
                            taker_base_volume: data.taker_base_volume,
                            maker_quote_volume: data.maker_quote_volume,
                            maker_base_volume: data.maker_base_volume,
                        };
                        candlestick = Some(c.clone());
                        c
                    }
                }
            }
        };
        Box::pin(stream)
    }

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

    #[error(transparent)]
    FractionError(#[from] FractionError),
}
