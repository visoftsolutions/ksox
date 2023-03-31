use std::{
    io::{Error, ErrorKind},
    time::Duration,
};

use axum::{
    extract::{Query, State},
    response::sse::{Event, Sse},
};
use futures::{stream::Stream, StreamExt, TryStreamExt};
use tokio::select;

use super::{Request, DepthResponse};
use crate::models::AppState;

pub async fn root(
    State(state): State<AppState>,
    Query(params): Query<Request>,
) -> Sse<impl Stream<Item = Result<Event, std::io::Error>>> {
    let stream = async_stream::try_stream! {
        let mut resp = DepthResponse::new();
        let resp_ref = &mut resp;
        let mut sells_stream = state.orders_manager.subscribe_for_orderbook(params.base_asset_id, params.quote_asset_id, params.precision, params.limit).await
            .map_err(|err| Error::new(ErrorKind::BrokenPipe, err))?;
        let mut buys_stream = state.orders_manager.subscribe_for_orderbook_opposite(params.quote_asset_id, params.base_asset_id, params.precision, params.limit).await
            .map_err(|err| Error::new(ErrorKind::BrokenPipe, err))?;

        loop {
            select! {
                Some(e) = sells_stream.next() => {
                    let element = e.map_err(|err| Error::new(ErrorKind::BrokenPipe, err)).unwrap_or_default();
                    resp_ref.sells = element;
                    yield Event::default().json_data(resp_ref.to_owned()).map_err(Error::from);
                },
                Some(e) = buys_stream.next() => {
                    let element = e.map_err(|err| Error::new(ErrorKind::BrokenPipe, err)).unwrap_or_default();
                    resp_ref.buys = element;
                    yield Event::default().json_data(resp_ref.to_owned()).map_err(Error::from);
                },
                else => break,
            }
        }
    }
    .map(|a| a.and_then(|b| b))
    .inspect_err(|err| tracing::error!("{err}"));

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(10))
            .text("keep-alive-text"),
    )
}
