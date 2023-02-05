use axum::{extract::State, Json};
use database::{projections::spot::asset::Asset, traits::manager::Manager};
use futures::StreamExt;

use crate::{api::AppError, AppState};

pub async fn root(State(state): State<AppState>) -> Result<Json<Vec<Asset>>, AppError> {
    let mut stream = state.assets_manager.get_all();
    let mut vec = Vec::<Asset>::new();
    while let Some(res) = stream.next().await {
        vec.push(res?);
    }
    Ok(Json(vec))
}
