use axum::response::sse::{Event, Sse};
use futures::{stream, Stream};
use std::{convert::Infallible, time::Duration};
use tokio_stream::StreamExt;

pub async fn trades() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let data = "hi from trades!";

    let stream = stream::repeat_with(move || Event::default().data(data))
        .map(Ok)
        .throttle(Duration::from_secs(1));

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}
