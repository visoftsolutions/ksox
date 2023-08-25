use std::{
    io::{Error, ErrorKind},
    ops::Deref,
    time::Duration,
};

use axum::{
    extract::State,
    response::sse::{Event, Sse},
};
use futures::stream::Stream;
use tokio_stream::StreamExt;

use crate::{
    api::{auth::models::UserId, private::transfers::DisplayTransfer},
    models::AppState,
};

pub async fn root(
    State(state): State<AppState>,
    user_id: UserId,
) -> Sse<impl Stream<Item = Result<Event, std::io::Error>>> {
    let stream = async_stream::stream! {
        let mut stream = state.transfers_notification_manager.subscribe_only_external_to_user(*user_id).await
            .map_err(|err| Error::new(ErrorKind::BrokenPipe, err))?;
        while let Some(element) = stream.next().await {
            if let Ok(t) = element {
                let result: Vec<DisplayTransfer> = t.iter().map(|x| DisplayTransfer::from_extended_transfer(*user_id.deref(), x.to_owned())).collect();
                yield Event::default().json_data(result).map_err(Error::from);
            } else {
                break;
            }
        }
    };

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(10))
            .text("keep-alive-text"),
    )
}
