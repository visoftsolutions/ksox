mod sse;

use axum::{extract::State, routing::get, Json, Router};
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
pub struct Request {
    pub quote_asset_id: Uuid,
    pub base_asset_id: Uuid,
    pub precision: i32,
    pub limit: i64,
}

pub async fn root(
    State(state): State<AppState>,
    Json(payload): Json<Request>,
) -> Result<Json<Vec<PriceLevel>>, AppError> {
    let mut stream = state
        .orders_manager
        .get_orderbook(
            payload.quote_asset_id,
            payload.base_asset_id,
            payload.precision,
            payload.limit,
        )
        .map(|f| f.and_then(TryInto::<PriceLevel>::try_into));

    let mut vec = Vec::<PriceLevel>::new();
    while let Some(res) = stream.next().await {
        vec.push(res?);
    }
    Ok(Json(vec))
}
