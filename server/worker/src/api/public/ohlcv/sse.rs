use std::{
    io::{Error, ErrorKind},
    time::Duration,
};

use axum::{
    extract::{Query, State},
    response::sse::{Event, Sse},
};
use futures::{stream::Stream, TryStreamExt};
use tokio_stream::StreamExt;

use super::Request;
use crate::{models::AppState, ohlcv::OhlcvEngine};

pub async fn root(
    State(state): State<AppState>,
    Query(params): Query<Request>,
) -> Sse<impl Stream<Item = Result<Event, std::io::Error>>> {
    let stream = async_stream::try_stream! {
        let ohlcv_engine = OhlcvEngine::new(state.database);
        let mut stream = ohlcv_engine.subscribe(params.quote_asset_id, params.base_asset_id, params.kind, params.reference_point, params.span).await;
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
