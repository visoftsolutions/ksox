mod sse;

use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};
use database::{sqlx::types::Uuid, types::PriceLevel};
use futures::StreamExt;
use serde::Deserialize;

use crate::{api::AppError, models::AppState};

pub fn router(app_state: &AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/sse", get(sse::root))
        .with_state(app_state.clone())
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub quote_asset_id: Uuid,
    pub base_asset_id: Uuid,
    pub precision: i32,
    pub limit: i64,
}

pub async fn root(
    State(state): State<AppState>,
    Query(params): Query<Request>,
) -> Result<Json<Vec<PriceLevel>>, AppError> {
    let mut stream = state
        .orders_manager
        .get_orderbook(
            params.quote_asset_id,
            params.base_asset_id,
            params.precision,
            params.limit,
        )
        .map(|f| f.and_then(TryInto::<PriceLevel>::try_into));

    let mut vec = Vec::<PriceLevel>::new();
    while let Some(res) = stream.next().await {
        vec.push(res?);
    }
    Ok(Json(vec))
}
