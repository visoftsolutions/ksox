mod sse;
use std::ops::Deref;

use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use tokio_stream::StreamExt;
use uuid::Uuid;

use crate::{
    api::{auth::models::UserId, AppError},
    models::AppState,
};

use super::DisplayTransfer;

pub fn router(app_state: &AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/sse", get(sse::root))
        .with_state(app_state.clone())
}

#[derive(Deserialize)]
pub struct Params {
    pub other_user_id: Uuid,
    pub limit: i64,
    pub offset: i64,
}

pub async fn root(
    State(state): State<AppState>,
    user_id: UserId,
    Query(params): Query<Params>,
) -> Result<Json<Vec<DisplayTransfer>>, AppError> {
    let mut stream = state
        .transfers_manager
        .get_specific_for_user_id(*user_id, params.other_user_id, params.limit, params.offset)
        .map(|t| {
            Ok::<DisplayTransfer, sqlx::Error>(DisplayTransfer::from_extended_transfer(
                *user_id.deref(),
                t?,
            ))
        });
    let mut vec = Vec::<DisplayTransfer>::new();
    while let Some(res) = stream.next().await {
        vec.push(res?);
    }
    Ok(Json(vec))
}
