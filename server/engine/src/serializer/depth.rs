use axum::response::sse::{Sse, Event};
use std::{convert::Infallible, time::Duration};
use futures::{stream, Stream};
use tokio_stream::StreamExt;

pub async fn depth() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let data = "hi from depth!";

    let stream = stream::repeat_with(move || Event::default().data(data))
        .map(Ok)
        .throttle(Duration::from_secs(1));
    
    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}
