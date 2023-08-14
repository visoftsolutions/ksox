use axum::{extract::State, Json};
use chrono::{DateTime, Utc};
use evm::address::Address;
use fraction::Fraction;
use serde::{Deserialize, Serialize};

use crate::{
    api::{auth::models::UserId, AppError},
    blockchain_base::WithdrawPermitRequest,
    models::AppState,
};

#[derive(Deserialize, Serialize, Debug)]
pub struct Request {
    pub spender: Address,
    pub asset: Address,
    pub amount: Fraction,
    pub deadline: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct Response {
    pub response: String,
}

pub async fn root(
    State(mut state): State<AppState>,
    _user_id: UserId,
    Json(payload): Json<Request>,
) -> Result<Json<Response>, AppError> {
    tracing::info!("{:?}", payload);
    let response = state
        .blockchain_client
        .withdraw(WithdrawPermitRequest {
            spender: payload.spender.to_string(),
            asset: payload.asset.to_string(),
            amount: serde_json::to_string(&payload.amount)?,
            deadline: payload.deadline.to_string(),
        })
        .await?
        .into_inner();

    Ok(Json(Response {
        response: response.signature,
    }))
}
