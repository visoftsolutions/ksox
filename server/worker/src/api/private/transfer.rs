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
    pub from_user_id: Uuid,
    pub to_user_id: Uuid,
    pub asset_id: Uuid,
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
    let from_valut_id = state
        .valuts_manager
        .get_for_user_asset(payload.from_user_id, payload.asset_id)
        .await?
        .id;
    let to_valut_id = state
        .valuts_manager
        .get_for_user_asset(payload.to_user_id, payload.asset_id)
        .await?
        .id;
    let asset = state.assets_manager.get_by_id(&payload.asset_id).await?;
    let response = state
        .engine_client
        .transfer(TransferRequest {
            from_valut_id: from_valut_id.to_string(),
            to_valut_id: to_valut_id.to_string(),
            asset_id: asset.id.to_string(),
            amount: serde_json::to_string(&payload.amount)?,
            fee_fraction: asset.transfer_fee.to_string(),
        })
        .await?
        .into_inner();

    Ok(Json(Response {
        response: format!("transfered {response:?}"),
    }))
}
