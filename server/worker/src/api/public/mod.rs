pub mod assets;
pub mod depth;
pub mod engagement;
pub mod ohlcv;
pub mod trades;
pub mod users;

use axum::{
    response::{sse::Event, Sse},
    routing::get,
    Router,
};
use chrono::{DateTime, Utc};
use fraction::Fraction;

use std::{convert::Infallible, time::Duration};

use futures::{stream, Stream};
use serde::Serialize;
use tokio_stream::StreamExt;

use super::Direction;
use crate::models::AppState;

pub fn router(app_state: &AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/sse", get(sse))
        .with_state(app_state.clone())
        .nest("/depth", depth::router(app_state))
        .nest("/ohlcv", ohlcv::router(app_state))
        .nest("/trades", trades::router(app_state))
        .nest("/assets", assets::router(app_state))
        .nest("/users", users::router(app_state))
        .nest("/engagement", engagement::router(app_state))
}

pub async fn root() -> &'static str {
    "Hello, World public!"
}

pub async fn sse() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = stream::repeat_with(|| {
        Event::default().data(format!("Hello, World public!, time: {}", Utc::now()))
    })
    .map(Ok)
    .throttle(Duration::from_secs(1));

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}

#[derive(Serialize)]
pub struct ResponseTrade {
    pub price: Fraction,
    pub volume: Fraction,
    pub time: DateTime<Utc>,
    pub direction: Direction,
}
