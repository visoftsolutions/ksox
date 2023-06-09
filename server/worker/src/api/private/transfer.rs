use axum::{extract::State, Json};
use fraction::Fraction;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    api::{auth::models::UserId, AppError},
    engine_base::TransferRequest,
    models::AppState,
};

#[derive(Deserialize, Serialize)]
pub struct Request {
    pub taker_id: Uuid,
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
        .transfer(TransferRequest {
            maker_id: (*user_id).to_string(),
            taker_id: payload.taker_id.to_string(),
            asset_id: payload.asset_id.to_string(),
            amount: serde_json::to_string(&payload.amount)?,
        })
        .await?
        .into_inner();

    Ok(Json(Response {
        response: format!("transfered {response:?}"),
    }))
}
