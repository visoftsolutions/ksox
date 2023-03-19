use std::{
    io::{Error, ErrorKind},
    time::Duration,
};

use axum::{
    extract::State,
    response::sse::{Event, Sse},
    Json,
};
use chrono::{DateTime, Utc};
use database::{sqlx::types::Uuid, types::CandlestickType};
use futures::{stream::Stream, TryStreamExt};
use serde::Deserialize;
use tokio_stream::StreamExt;

use crate::{models::AppState, ohlcv::OhlcvEngine};

// TODO define macro for this
#[derive(Deserialize)]
pub struct RequestPartial {
    quote_asset_id: Uuid,
    base_asset_id: Uuid,
    kind: Option<CandlestickType>,
    reference_point: DateTime<Utc>,
    span: i64,
}

#[derive(Debug)]
pub struct Request {
    quote_asset_id: Uuid,
    base_asset_id: Uuid,
    kind: CandlestickType,
    reference_point: DateTime<Utc>,
    span: i64,
}

impl RequestPartial {
    fn insert_defaults(self) -> Request {
        Request {
            quote_asset_id: self.quote_asset_id,
            base_asset_id: self.base_asset_id,
            kind: self.kind.unwrap_or(CandlestickType::Interval),
            reference_point: self.reference_point,
            span: self.span,
        }
    }
}

// Return ohlcv stream
pub async fn root(
    State(state): State<AppState>,
    Json(payload): Json<RequestPartial>,
) -> Sse<impl Stream<Item = Result<Event, std::io::Error>>> {
    let payload = payload.insert_defaults();
    tracing::info!("{}", payload.reference_point);
    let stream = async_stream::try_stream! {
        let ohlcv_engine = OhlcvEngine::new(state.database);
        let mut stream = ohlcv_engine.subscribe(payload.quote_asset_id, payload.base_asset_id, payload.kind, payload.reference_point, payload.span).await;
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
