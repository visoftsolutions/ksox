use std::{
    io::{Error, ErrorKind},
    time::Duration,
};

use axum::{
    extract::State,
    response::sse::{Event, Sse},
};
use futures::{stream::Stream, TryStreamExt};
use tokio_stream::StreamExt;

use crate::{api::auth::models::UserId, models::AppState};

pub async fn root(
    State(state): State<AppState>,
    user_id: UserId,
) -> Sse<impl Stream<Item = Result<Event, std::io::Error>>> {
    let stream = async_stream::try_stream! {
        let mut stream = state.orders_manager.subscribe_active_for_user(*user_id).await
            .map_err(|err| Error::new(ErrorKind::BrokenPipe, err))?;
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
