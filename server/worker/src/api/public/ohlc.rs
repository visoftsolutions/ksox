use std::{convert::Infallible, time::Duration};

use axum::response::sse::{Event, Sse};
use futures::stream::{self, Stream};
use tokio_stream::StreamExt as _;

pub async fn root() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    // A `Stream` that repeats an event every second
    let stream = stream::repeat_with(|| Event::default().data("hi it is ohlc endpoint"))
        .map(Ok)
        .throttle(Duration::from_secs(1));

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}
