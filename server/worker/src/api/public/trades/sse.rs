use std::{
    io::{Error, ErrorKind},
    time::Duration,
};

use axum::{
    extract::{Query, State},
    response::sse::{Event, Sse},
};
use futures::{stream::Stream, StreamExt};
use serde::Deserialize;
use uuid::Uuid;

use crate::{models::AppState, database::projections::trade::Trade};

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
                    t.inverse()
                } else {
                    t
                }
            }).collect::<Vec<Trade>>();
            yield Event::default().json_data(trades).map_err(Error::from);
        }
    };

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(10))
            .text("keep-alive-text"),
    )
}
