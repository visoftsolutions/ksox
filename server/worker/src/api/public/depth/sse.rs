use std::{
    io::{self},
    time::Duration,
};

use axum::{
    extract::{Query, State},
    response::sse::{Event, Sse},
};
use fraction::{num_traits::Inv, Fraction};
use futures::{stream::Stream, StreamExt};
use tokio::select;

use super::{Request, Response};
use crate::models::AppState;

pub async fn root(
    State(state): State<AppState>,
    Query(params): Query<Request>,
) -> Sse<impl Stream<Item = Result<Event, std::io::Error>>> {
    let stream = async_stream::try_stream! {
        let precision = Fraction::from(params.precision).inv();

        let mut resp = Response::new();
        let resp_ref = &mut resp;

        let mut buys_stream_subscribe = state.orders_notification_manager.subscribe_to_orderbook(params.quote_asset_id, params.base_asset_id, precision.to_owned(), params.limit).await?;
        let mut sells_stream_subscribe = state.orders_notification_manager.subscribe_to_orderbook_opposite(params.quote_asset_id, params.base_asset_id, precision.to_owned(), params.limit).await?;

        let mut buys_stream = state.orders_manager.get_orderbook(params.quote_asset_id, params.base_asset_id, precision.to_owned()).take(params.limit);
        let mut sells_stream = state.orders_manager.get_orderbook_opposite(params.quote_asset_id, params.base_asset_id, precision.to_owned()).take(params.limit);

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
                    resp_ref.sells = e.unwrap_or_default();
                },
                Some(e) = buys_stream_subscribe.next() => {
                    resp_ref.buys = e.unwrap_or_default();
                },
                else => break,
            }
            yield Event::default().json_data(resp_ref.to_owned()).map_err(io::Error::from);
        }
    }
    .map(|a| a.and_then(|b| b));

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(10))
            .text("keep-alive-text"),
    )
}
