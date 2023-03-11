use std::{
    io::{Error, ErrorKind},
    time::Duration,
};

use axum::{
    extract::State,
    response::sse::{Event, Sse},
    Json,
};
use database::sqlx::types::Uuid;
use futures::{stream::Stream, StreamExt, TryStreamExt};
use serde::Deserialize;

use crate::models::AppState;

// TODO define macro for this
#[derive(Deserialize)]
pub struct RequestPartial {
    pub quote_asset_id: Uuid,
    pub base_asset_id: Uuid,
    pub precision: Option<i32>,
    pub limit: Option<i64>,
}

pub struct Request {
    pub quote_asset_id: Uuid,
    pub base_asset_id: Uuid,
    pub precision: i32,
    pub limit: i64,
}

impl RequestPartial {
    fn insert_defaults(self) -> Request {
        Request {
            quote_asset_id: self.quote_asset_id,
            base_asset_id: self.base_asset_id,
            precision: self.precision.unwrap_or(2),
            limit: self.limit.unwrap_or(10),
        }
    }
}

// Return stream of trades from db
pub async fn root(
    State(state): State<AppState>,
    Json(payload): Json<RequestPartial>,
) -> Sse<impl Stream<Item = Result<Event, std::io::Error>>> {
    let payload = payload.insert_defaults();
    let stream = async_stream::try_stream! {
        let mut stream = state.orders_manager.subscribe_for_orderbook(payload.quote_asset_id, payload.base_asset_id, payload.precision, payload.limit).await
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
