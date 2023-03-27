mod sse;

use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};
use chrono::{DateTime, Utc};
use database::{
    projections::spot::candlestick::Candlestick, sqlx::types::Uuid, types::CandlestickType,
};
use serde::Deserialize;

use crate::{api::AppError, models::AppState, ohlcv::OhlcvEngine};

pub fn router(app_state: &AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/sse", get(sse::root))
        .with_state(app_state.clone())
}

#[derive(Deserialize)]
pub struct Request {
    quote_asset_id: Uuid,
    base_asset_id: Uuid,
    kind: CandlestickType,
    reference_point: DateTime<Utc>,
    span: i64,
}

pub async fn root(
    State(state): State<AppState>,
    Query(params): Query<Request>,
) -> Result<Json<Option<Candlestick>>, AppError> {
    let ohlcv_engine = OhlcvEngine::new(state.database);
    Ok(Json(
        ohlcv_engine
            .get(
                params.quote_asset_id,
                params.base_asset_id,
                params.kind,
                params.reference_point,
                params.span,
            )
            .await?,
    ))
}
