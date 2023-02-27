use std::{convert::Infallible, ops::Deref, time::Duration};

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
    let stream = async_stream::stream! {
        let sub = state.valuts_manager.get_and_subscribe_for_user(*user_id.deref()).await;
        match sub {
            Ok(mut stream) => {
                while let Some(element) = stream.next().await {
                    match element {
                        Ok(element) => {
                            match Event::default().json_data(element) {
                                Ok(data) => {
                                    yield Ok(data);
                                },
                                Err(err) => {
                                    tracing::error!("{err}");
                                }
                            }
                        },
                        Err(err) => {
                            tracing::error!("{err}");
                        }
                    }
                }
            },
            Err(err) => {
                tracing::error!("{err}");
            }
        }
    };

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}
