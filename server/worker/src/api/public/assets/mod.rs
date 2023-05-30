pub mod search_asset_pair;

use axum::{extract::State, routing::get, Json, Router};
use tokio_stream::StreamExt;

use crate::{api::AppError, database::projections::asset::Asset, models::AppState};

pub fn router(app_state: &AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/search_asset_pair", get(search_asset_pair::root))
        .with_state(app_state.clone())
}

// Return all assets that exist in db
pub async fn root(State(state): State<AppState>) -> Result<Json<Vec<Asset>>, AppError> {
    let mut stream = state.assets_manager.get_all();
    let mut vec = Vec::<Asset>::new();
    while let Some(res) = stream.next().await {
        vec.push(res?);
    }
    Ok(Json(vec))
}
