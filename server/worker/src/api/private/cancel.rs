use axum::{extract::State, Json};
use database::sqlx::types::Uuid;
use serde::Deserialize;

use crate::{
    api::{auth::models::UserId, AppError},
    engine_base::CancelRequest,
    models::AppState,
};

#[derive(Deserialize)]
pub struct Request {
    pub order_id: Uuid,
}

pub async fn root(
    State(mut state): State<AppState>,
    _user_id: UserId,
    Json(payload): Json<Request>,
) -> Result<String, AppError> {
    let response = state
        .engine_client
        .cancel(CancelRequest {
            order_id: payload.order_id.to_string(),
        })
        .await?
        .into_inner();
    Ok(format!("canceled with msg: {:?}", response))
}
