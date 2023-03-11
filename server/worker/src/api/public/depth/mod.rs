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

// TODO define macro for this
#[derive(Deserialize)]
pub struct RequestPartial {
    pub quote_asset_id: Uuid,
    pub base_asset_id: Uuid,
    pub precision: Option<i32>,
    pub limit: Option<i64>,
}

pub struct Request {
    pub quote_asset_id: Uuid,
    pub base_asset_id: Uuid,
    pub precision: i32,
    pub limit: i64,
}

impl RequestPartial {
    fn insert_defaults(self) -> Request {
        Request {
            quote_asset_id: self.quote_asset_id,
            base_asset_id: self.base_asset_id,
            precision: self.precision.unwrap_or(2),
            limit: self.limit.unwrap_or(10),
        }
    }
}

pub async fn root(
    State(state): State<AppState>,
    Json(payload): Json<RequestPartial>,
) -> Result<Json<Vec<PriceLevel>>, AppError> {
    let payload = payload.insert_defaults();
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
