use std::{
    io::{Error, ErrorKind},
    time::Duration,
};

use axum::{
    extract::{Query, State},
    response::sse::{Event, Sse},
};
use database::sqlx::types::Uuid;
use futures::{stream::Stream, StreamExt, TryStreamExt};
use serde::Deserialize;

use crate::models::AppState;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub quote_asset_id: Uuid,
    pub base_asset_id: Uuid,
}

// Return stream of trades from db
pub async fn root(
    State(state): State<AppState>,
    Query(params): Query<Request>,
) -> Sse<impl Stream<Item = Result<Event, std::io::Error>>> {
    let stream = async_stream::try_stream! {
        let mut stream = state.trades_manager.subscribe_for_asset_pair(params.quote_asset_id, params.base_asset_id).await
            .map_err(|err| Error::new(ErrorKind::BrokenPipe, err))?;
        while let Some(element) = stream.next().await {
            yield Event::default().json_data(
                element.map_err(|err| Error::new(ErrorKind::BrokenPipe, err))?
            ).map_err(Error::from);
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
