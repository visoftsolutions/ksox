use axum::{extract::State, Json};
use evm::address::Address;
use fraction::Fraction;
use serde::{Deserialize, Serialize};

use crate::{
    api::{auth::models::UserId, AppError},
    blockchain_base::WithdrawRequest,
    models::AppState,
};

#[derive(Deserialize, Serialize, Debug)]
pub struct Request {
    pub maker_address: Address,
    pub taker_address: Address,
    pub asset_address: Address,
    pub amount: Fraction,
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
        .withdraw(WithdrawRequest {
            maker_address: payload.maker_address.to_string(),
            taker_address: payload.taker_address.to_string(),
            asset_address: payload.asset_address.to_string(),
            amount: serde_json::to_string(&payload.amount)?,
        })
        .await?
        .into_inner();

    Ok(Json(Response {
        response: format!("withdrawn {response:?}"),
    }))
}
