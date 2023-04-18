use axum::{extract::State, Json};
use fraction::Fraction;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    api::{auth::models::UserId, AppError},
    engine_base::SubmitRequest,
    models::AppState,
};

#[derive(Deserialize)]
pub struct Request {
    pub quote_asset_id: Uuid,
    pub base_asset_id: Uuid,
    pub price: Fraction,
    pub quote_asset_volume: Fraction,
}

pub async fn root(
    State(mut state): State<AppState>,
    user_id: UserId,
    Json(payload): Json<Request>,
) -> Result<String, AppError> {
    let response = state
        .engine_client
        .submit(SubmitRequest {
            user_id: (*user_id).to_string(),
            quote_asset_id: payload.quote_asset_id.to_string(),
            base_asset_id: payload.base_asset_id.to_string(),
            price: serde_json::to_string(&payload.price)?,
            quote_asset_volume: serde_json::to_string(&payload.quote_asset_volume)?,
        })
        .await?
        .into_inner();
    Ok(format!("submitted {response:?}"))
}
