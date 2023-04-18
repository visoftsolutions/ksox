use axum::{extract::State, Json};
use fraction::Fraction;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    api::{auth::models::UserId, AppError},
    models::AppState,
};

#[derive(Deserialize)]
pub struct Request {
    pub asset_id: Uuid,
    pub amount: Fraction,
}

pub async fn root(
    State(state): State<AppState>,
    user_id: UserId,
    Json(payload): Json<Request>,
) -> Result<String, AppError> {
    let user = state.users_manager.get_by_id(*user_id).await?;
    user.burn(
        &state.valuts_manager,
        payload.asset_id,
        payload.amount.clone(),
    )
    .await?;
    Ok(format!(
        "burned {} of {} for {}",
        payload.amount, payload.asset_id, user_id
    ))
}
