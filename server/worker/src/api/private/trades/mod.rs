mod sse;

use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};
use database::projections::spot::trade::Trade;
use tokio_stream::StreamExt;

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
) -> Result<Json<Vec<Trade>>, AppError> {
    let mut stream = state
        .trades_manager
        .get_for_taker(*user_id, params.limit, params.offset);
    let mut vec = Vec::<Trade>::new();
    while let Some(res) = stream.next().await {
        vec.push(res?);
    }
    Ok(Json(vec))
}
