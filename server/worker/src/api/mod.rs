pub mod auth;
pub mod private;
pub mod public;

use std::{convert::Infallible, time::Duration};

use axum::{
    response::{sse::Event, IntoResponse, Response, Sse},
    TypedHeader,
};
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

impl Default for Pagination {
    fn default() -> Self {
        Self {
            limit: 10,
            offset: 0,
        }
    }
}

pub async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn sse(
    TypedHeader(user_agent): TypedHeader<headers::UserAgent>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    println!("`{}` connected", user_agent.as_str());

    // A `Stream` that repeats an event every second
    let stream = stream::repeat_with(|| {
        Event::default().data(format!("Hello, World!, time: {}", Utc::now().to_string()))
    })
    .map(Ok)
    .throttle(Duration::from_secs(1));

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}
