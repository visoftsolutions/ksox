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

use crate::{api::auth::models::UserId, models::AppState};

pub async fn root(
    State(state): State<AppState>,
    user_id: UserId,
) -> Sse<impl Stream<Item = Result<Event, std::io::Error>>> {
    let stream = async_stream::stream! {
        let user = state.users_manager.get_by_id(*user_id).await.map_err(|err| Error::new(ErrorKind::BrokenPipe, err))?;
        let mut stream = state.deposits_notification_manager.subscribe_for_user(user.address).await
            .map_err(|err| Error::new(ErrorKind::BrokenPipe, err))?;
        while let Some(element) = stream.next().await {
            if !element.is_empty() {
                yield Event::default().json_data(element.clone()).map_err(Error::from);
            }
        }
    };

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(10))
            .text("keep-alive-text"),
    )
}
