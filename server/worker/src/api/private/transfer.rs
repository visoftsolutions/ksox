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
    pub taker: Uuid,
    pub asset: Uuid,
    pub volume: Fraction,
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
            maker: (*user_id).to_string(),
            taker: payload.taker.to_string(),
            asset: payload.asset.to_string(),
            volume: serde_json::to_string(&payload.volume)?,
        })
        .await?
        .into_inner();

    Ok(Json(Response {
        response: format!("transfered {response:?}"),
    }))
}
