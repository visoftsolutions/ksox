mod sse;

use axum::{extract::State, routing::get, Json, Router};
use database::{projections::spot::trade::Trade, sqlx::types::Uuid};
use serde::Deserialize;
use tokio_stream::StreamExt;

use crate::{
    api::{AppError, Pagination},
    models::AppState,
};

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
    pub pagination: Option<Pagination>,
}

pub struct Request {
    pub quote_asset_id: Uuid,
    pub base_asset_id: Uuid,
    pub pagination: Pagination,
}

impl RequestPartial {
    fn insert_defaults(self) -> Request {
        Request {
            quote_asset_id: self.quote_asset_id,
            base_asset_id: self.base_asset_id,
            pagination: self.pagination.unwrap_or_default(),
        }
    }
}

pub async fn root(
    State(state): State<AppState>,
    Json(payload): Json<RequestPartial>,
) -> Result<Json<Vec<Trade>>, AppError> {
    let payload = payload.insert_defaults();
    let mut stream = state.trades_manager.get_for_asset_pair(
        payload.quote_asset_id,
        payload.base_asset_id,
        payload.pagination.limit,
        payload.pagination.offset,
    );
    let mut vec = Vec::<Trade>::new();
    while let Some(res) = stream.next().await {
        vec.push(res?);
    }
    Ok(Json(vec))
}
