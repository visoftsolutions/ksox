use std::{convert::Infallible, time::Duration};

use axum::{
    extract::State,
    response::sse::{Event, Sse},
};
use futures::stream::{self, Stream};
use tokio_stream::StreamExt;

use crate::{api::auth::models::UserId, models::AppState};

pub async fn root(
    State(state): State<AppState>,
    user_id: UserId,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    // A `Stream` that repeats an event every second
    let stream = stream::repeat_with(|| Event::default().data("hi it is depth endpoint"))
        .map(Ok)
        .throttle(Duration::from_secs(1));

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}
