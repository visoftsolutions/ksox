mod sse;

use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use tokio::select;
use uuid::Uuid;

use crate::{api::AppError, database::projections::order::PriceLevel, models::AppState};

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

#[derive(Deserialize, Serialize, Clone)]
pub struct DepthResponse {
    pub sells: Vec<PriceLevel>,
    pub buys: Vec<PriceLevel>,
}

impl DepthResponse {
    fn new() -> Self {
        Self {
            sells: Vec::new(),
            buys: Vec::new(),
        }
    }
}

pub async fn root(
    State(state): State<AppState>,
    Query(params): Query<Request>,
) -> Result<Json<DepthResponse>, AppError> {
    let mut resp = DepthResponse::new();

    let mut buys_stream = state
        .orders_manager
        .get_orderbook(
            params.quote_asset_id,
            params.base_asset_id,
            params.precision,
            params.limit,
        )
        .map(|f| f.and_then(TryInto::<PriceLevel>::try_into));

    let mut sells_stream = state
        .orders_manager
        .get_orderbook_opposite(
            params.quote_asset_id,
            params.base_asset_id,
            params.precision,
            params.limit,
        )
        .map(|f| f.and_then(TryInto::<PriceLevel>::try_into));

    loop {
        select! {
            Some(e) = sells_stream.next() => {
                resp.sells.push(e?);
            },
            Some(e) = buys_stream.next() => {
                resp.buys.push(e?);
            },
            else => break,
        }
    }

    Ok(Json(resp))
}
