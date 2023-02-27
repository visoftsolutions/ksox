use std::{convert::Infallible, rc::Rc, thread::yield_now, time::Duration};

use axum::{
    extract::State,
    response::sse::{Event, Sse},
    Json,
};
use database::sqlx::types::Uuid;
use futures::stream::{self, Stream};
use serde::Deserialize;
use tokio_stream::StreamExt;

use crate::models::AppState;

#[derive(Deserialize)]
pub struct Request {
    pub quote_asset_id: Uuid,
    pub base_asset_id: Uuid,
}

// Return stream of trades from db
pub async fn root(
    State(state): State<AppState>,
    Json(payload): Json<Request>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    // A `Stream` that repeats an event every second
    let stream = async_stream::stream! {
        let sub = state.trades_manager.get_and_subscribe_for_asset_pair(payload.quote_asset_id, payload.base_asset_id).await;
        match sub {
            Ok(mut stream) => {
                while let Some(element) = stream.next().await {
                    match element {
                        Ok(element) => {
                            match Event::default().json_data(element) {
                                Ok(data) => {
                                    yield Ok(data);
                                },
                                Err(err) => {
                                    tracing::error!("{err}");
                                }
                            }
                        },
                        Err(err) => {
                            tracing::error!("{err}");
                        }
                    }
                }
            },
            Err(err) => {
                tracing::error!("{err}");
            }
        }
    };

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}
