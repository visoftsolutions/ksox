use std::{
    io::{Error, ErrorKind},
    time::Duration,
};

use axum::{
    extract::{Query, State},
    response::sse::{Event, Sse},
};
use fraction::num_traits::Inv;
use futures::{stream::Stream, StreamExt};
use serde::Deserialize;
use uuid::Uuid;

use super::Direction;
use crate::{api::public::ResponseTrade, models::AppState};

#[derive(Deserialize)]
pub struct Request {
    pub quote_asset_id: Uuid,
    pub base_asset_id: Uuid,
}

// Return stream of trades from db
pub async fn root(
    State(state): State<AppState>,
    Query(params): Query<Request>,
) -> Sse<impl Stream<Item = Result<Event, std::io::Error>>> {
    let stream = async_stream::stream! {
        let mut stream = state.trades_notification_manager.subscribe_to_asset_pair(params.quote_asset_id, params.base_asset_id).await
            .map_err(|err| Error::new(ErrorKind::BrokenPipe, err))?;
        while let Some(element) = stream.next().await {
            let trades = element.into_iter().map(|t| {
                if t.is_opposite(params.quote_asset_id, params.base_asset_id) {
                    ResponseTrade { price: t.price.inv(), volume: t.maker_quote_volume , time: t.created_at, direction: Direction::Sell }
                } else {
                    ResponseTrade { price: t.price, volume: t.taker_quote_volume , time: t.created_at, direction: Direction::Buy }
                }
            }).rev().collect::<Vec<ResponseTrade>>();
            yield Event::default().json_data(trades).map_err(Error::from);
        }
    };

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(10))
            .text("keep-alive-text"),
    )
}
