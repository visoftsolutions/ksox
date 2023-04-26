mod sse;

use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};
use tokio_stream::StreamExt;

use super::ResponseTrade;
use crate::{
    api::{auth::models::UserId, AppError, Pagination},
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
    Query(params): Query<Pagination>,
) -> Result<Json<Vec<ResponseTrade>>, AppError> {
    let mut stream = state
        .trades_manager
        .get_for_user_id(*user_id, params.limit, params.offset);
    let mut vec = Vec::<ResponseTrade>::new();
    while let Some(res) = stream.next().await {
        vec.push(ResponseTrade::from(res?, *user_id));
    }
    Ok(Json(vec))
}
