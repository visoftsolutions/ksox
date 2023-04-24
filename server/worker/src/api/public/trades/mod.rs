mod sse;

use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use tokio_stream::StreamExt;
use uuid::Uuid;

use crate::{api::AppError, database::projections::trade::Trade, models::AppState};

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
    pub limit: i64,
    pub offset: i64,
}

pub async fn root(
    State(state): State<AppState>,
    Query(params): Query<Request>,
) -> Result<Json<Vec<Trade>>, AppError> {
    let mut stream = state.trades_manager.get_for_asset_pair(
        params.quote_asset_id,
        params.base_asset_id,
        params.limit,
        params.offset,
    );
    let mut vec = Vec::<Trade>::new();
    while let Some(res) = stream.next().await {
        let t = res?;
        if t.is_opposite(params.quote_asset_id, params.base_asset_id) {
            vec.push(t.inverse());
        } else {
            vec.push(t);
        }
    }
    Ok(Json(vec))
}
