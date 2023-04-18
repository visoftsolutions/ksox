use std::{
    io::{Error, ErrorKind},
    time::Duration,
};

use axum::{
    extract::{Query, State},
    response::sse::{Event, Sse},
};
use futures::{stream::Stream, StreamExt};
use tokio::select;

use super::{DepthResponse, Request};
use crate::{database::projections::order::PriceLevel, models::AppState};

// TODO refactor this endpoint some more elegant way :) -- better error handling no unwraps
pub async fn root(
    State(state): State<AppState>,
    Query(params): Query<Request>,
) -> Sse<impl Stream<Item = Result<Event, std::io::Error>>> {
    let stream = async_stream::try_stream! {
        let mut resp = DepthResponse::new();
        let resp_ref = &mut resp;
        let mut buys_stream_subscribe = state.orders_notification_manager.subscribe_to_orderbook(params.quote_asset_id, params.base_asset_id, params.precision, params.limit).await
            .map_err(|err| Error::new(ErrorKind::BrokenPipe, err))?;
        let mut sells_stream_subscribe = state.orders_notification_manager.subscribe_to_orderbook_opposite(params.quote_asset_id, params.base_asset_id, params.precision, params.limit).await
            .map_err(|err| Error::new(ErrorKind::BrokenPipe, err))?;

        let mut buys_stream = state.orders_manager.get_orderbook(params.quote_asset_id,params.base_asset_id,params.precision,params.limit).map(|f| f.and_then(TryInto::<PriceLevel>::try_into));
        let mut sells_stream = state.orders_manager.get_orderbook_opposite(params.quote_asset_id, params.base_asset_id, params.precision, params.limit).map(|f| f.and_then(TryInto::<PriceLevel>::try_into));

        loop {
            select! {
                Some(e) = sells_stream.next() => {
                    resp_ref.sells.push(e.unwrap_or_default());
                },
                Some(e) = buys_stream.next() => {
                    resp_ref.buys.push(e.unwrap_or_default());
                },
                else => break,
            }
        }

        loop {
            select! {
                Some(e) = sells_stream_subscribe.next() => {
                    resp_ref.sells = e.into_iter().filter_map(|price_level| {
                        if price_level.price.is_some() || price_level.volume.is_some() {
                            Some(PriceLevel { price: price_level.price.unwrap(), volume: price_level.volume.unwrap() })
                        } else {
                            None
                        }
                    }).collect();
                },
                Some(e) = buys_stream_subscribe.next() => {
                    resp_ref.buys = e.into_iter().filter_map(|price_level| {
                        if price_level.price.is_some() || price_level.volume.is_some() {
                            Some(PriceLevel { price: price_level.price.unwrap(), volume: price_level.volume.unwrap() })
                        } else {
                            None
                        }
                    }).collect();
                },
                else => break,
            }
            yield Event::default().json_data(resp_ref.to_owned()).map_err(Error::from);
        }
    }
    .map(|a| a.and_then(|b| b));

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(10))
            .text("keep-alive-text"),
    )
}
