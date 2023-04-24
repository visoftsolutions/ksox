mod sse;

use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};
use fraction::{num_traits::Inv, Fraction};
use futures::StreamExt;
use pricelevel::PriceLevel;
use serde::{Deserialize, Serialize};
use tokio::select;
use uuid::Uuid;

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
    pub precision: usize,
    pub limit: usize,
}

#[derive(Serialize, Clone)]
pub struct Response {
    pub sells: Vec<PriceLevel>,
    pub buys: Vec<PriceLevel>,
}

impl Response {
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
) -> Result<Json<Response>, AppError> {
    let mut resp = Response::new();
    let resp_ref = &mut resp;

    let precision = Fraction::from(params.precision).inv();
    let mut buys_stream = state
        .orders_manager
        .get_orderbook(
            params.quote_asset_id,
            params.base_asset_id,
            precision.to_owned(),
        )
        .take(params.limit);
    let mut sells_stream = state
        .orders_manager
        .get_orderbook_opposite(
            params.quote_asset_id,
            params.base_asset_id,
            precision.to_owned(),
        )
        .take(params.limit);

    loop {
        select! {
            Some(e) = sells_stream.next() => {
                resp_ref.sells.push(e.unwrap_or_default());
            },
            Some(e) = buys_stream.next() => {
                resp_ref.buys.push(e.unwrap_or_default());
            },
            else => break,
        }
    }

    Ok(Json(resp))
}
