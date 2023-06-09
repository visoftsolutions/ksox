mod sse;

use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};
use tokio_stream::StreamExt;

use crate::{
    api::{auth::models::UserId, AppError},
    database::projections::badge::Badge,
    models::AppState,
};

pub fn router(app_state: &AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/sse", get(sse::root))
        .with_state(app_state.clone())
}

pub async fn root(
    State(state): State<AppState>,
    user_id: UserId,
) -> Result<Json<Vec<Badge>>, AppError> {
    let mut stream = state.badges_manager.get_for_user_id(*user_id);
    let mut vec = Vec::<Badge>::new();
    while let Some(res) = stream.next().await {
        vec.push(res?);
    }
    Ok(Json(vec))
}
