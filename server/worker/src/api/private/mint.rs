use axum::{extract::State, Json};
use fraction::Fraction;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    api::{auth::models::UserId, AppError},
    engine_base::MintRequest,
    models::AppState,
};

#[derive(Deserialize)]
pub struct Request {
    pub asset_id: Uuid,
    pub amount: Fraction,
}

#[derive(Serialize)]
pub struct Response {
    pub response: String,
}

pub async fn root(
    State(mut state): State<AppState>,
    user_id: UserId,
    Json(payload): Json<Request>,
) -> Result<Json<Response>, AppError> {
    let response = state
        .engine_client
        .mint(MintRequest {
            user_id: (*user_id).to_string(),
            asset_id: payload.asset_id.to_string(),
            amount: serde_json::to_string(&payload.amount)?,
        })
        .await?
        .into_inner();

    Ok(Json(Response {
        response: format!("transfered {response:?}"),
    }))
}
