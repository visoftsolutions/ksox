use std::{
    io::{Error, ErrorKind},
    time::Duration,
};

use axum::{
    extract::{Query, State},
    response::sse::{Event, Sse},
};
use futures::stream::Stream;
use tokio_stream::StreamExt;

use super::Request;
use crate::{api::auth::models::UserId, models::AppState};

pub async fn root(
    State(state): State<AppState>,
    user_id: UserId,
    Query(params): Query<Request>,
) -> Sse<impl Stream<Item = Result<Event, std::io::Error>>> {
    let stream = async_stream::stream! {
        let mut stream = state.valuts_notification_manager.subscribe_for_user_asset(*user_id, params.asset_id).await
            .map_err(|err| Error::new(ErrorKind::BrokenPipe, err))?;
        while let Some(element) = stream.next().await {
            if !element.is_empty() {
                yield Event::default().json_data(element[0].clone()).map_err(Error::from);
            }
        }
    };

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(10))
            .text("keep-alive-text"),
    )
}
