mod sse;

use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};
use chrono::{DateTime, Utc};
use fraction::{num_traits::Inv, Fraction};
use serde::{Deserialize, Serialize};
use tokio_stream::StreamExt;
use uuid::Uuid;

use crate::{api::AppError, models::AppState};

pub fn router(app_state: &AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/sse", get(sse::root))
        .with_state(app_state.clone())
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    Buy,
    Sell,
}

#[derive(Deserialize)]
pub struct Request {
    pub quote_asset_id: Uuid,
    pub base_asset_id: Uuid,
    pub limit: i64,
    pub offset: i64,
}

#[derive(Serialize)]
pub struct Response {
    pub price: Fraction,
    pub volume: Fraction,
    pub time: DateTime<Utc>,
    pub direction: Direction,
}

pub async fn root(
    State(state): State<AppState>,
    Query(params): Query<Request>,
) -> Result<Json<Vec<Response>>, AppError> {
    let mut stream = state.trades_manager.get_for_asset_pair(
        params.quote_asset_id,
        params.base_asset_id,
        params.limit,
        params.offset,
    );
    let mut vec = Vec::<Response>::new();
    while let Some(res) = stream.next().await {
        let t = res?;
        if t.is_opposite(params.quote_asset_id, params.base_asset_id) {
            vec.push(Response {
                price: t.price.inv(),
                volume: t.maker_quote_volume,
                time: t.created_at,
                direction: Direction::Sell,
            });
        } else {
            vec.push(Response {
                price: t.price,
                volume: t.taker_quote_volume,
                time: t.created_at,
                direction: Direction::Buy,
            });
        }
    }
    Ok(Json(vec))
}
