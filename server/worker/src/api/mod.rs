pub mod auth;
pub mod private;
pub mod public;

use std::{convert::Infallible, time::Duration};

use axum::response::{sse::Event, IntoResponse, Response, Sse};
use chrono::Utc;
use futures::{stream, Stream};
use http::StatusCode;
use serde::Deserialize;
use tokio_stream::StreamExt;

pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

#[derive(Deserialize)]
pub struct Pagination {
    pub limit: i64,
    pub offset: i64,
}

pub async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn sse() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = stream::repeat_with(|| {
        Event::default().data(format!("Hello, World!, time: {}", Utc::now()))
    })
    .map(Ok)
    .throttle(Duration::from_secs(1));

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}
