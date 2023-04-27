mod sse;

use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};
use tokio_stream::StreamExt;

use super::ResponseOrder;
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
) -> Result<Json<Vec<ResponseOrder>>, AppError> {
    let mut stream = state
        .orders_manager
        .get_for_user(*user_id, params.limit, params.offset);
    let mut vec = Vec::<ResponseOrder>::new();
    while let Some(res) = stream.next().await {
        vec.push(ResponseOrder::from(res?));
    }
    Ok(Json(vec))
}
