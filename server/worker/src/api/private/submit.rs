use axum::{extract::State, Json};
use database::{sqlx::types::Uuid, types::Volume};
use serde::Deserialize;

use crate::{
    api::{auth::models::UserId, AppError},
    engine_base::SubmitRequest,
    models::AppState,
};

#[derive(Deserialize)]
pub struct Request {
    pub user_id: Uuid,
    pub quote_asset_id: Uuid,
    pub base_asset_id: Uuid,
    pub quote_asset_volume: Volume,
    pub base_asset_volume: Volume,
}

pub async fn root(
    State(mut state): State<AppState>,
    _user_id: UserId,
    Json(payload): Json<Request>,
) -> Result<String, AppError> {
    let response = state
        .engine_client
        .submit(SubmitRequest {
            user_id: payload.user_id.to_string(),
            quote_asset_id: payload.quote_asset_id.to_string(),
            base_asset_id: payload.base_asset_id.to_string(),
            quote_asset_volume: payload.quote_asset_volume.to_string(),
            base_asset_volume: payload.base_asset_volume.to_string(),
        })
        .await?
        .into_inner();
    Ok(format!("submitted with msg: {:?}", response))
}
