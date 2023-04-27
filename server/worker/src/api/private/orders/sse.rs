use std::{
    io::{Error, ErrorKind},
    time::Duration,
};

use axum::{
    extract::State,
    response::sse::{Event, Sse},
};
use futures::stream::Stream;
use tokio_stream::StreamExt;

use crate::{
    api::{auth::models::UserId, private::ResponseOrder},
    models::AppState,
};

pub async fn root(
    State(state): State<AppState>,
    user_id: UserId,
) -> Sse<impl Stream<Item = Result<Event, std::io::Error>>> {
    let stream = async_stream::stream! {
        let mut stream = state.orders_notification_manager.subscribe_to_user_id(*user_id).await
            .map_err(|err| Error::new(ErrorKind::BrokenPipe, err))?;
        while let Some(element) = stream.next().await {
            yield Event::default().json_data(element.into_iter().map(|f| ResponseOrder::from(f)).collect::<Vec<ResponseOrder>>()).map_err(Error::from);
        }
    };

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(10))
            .text("keep-alive-text"),
    )
}
