use axum::extract::{Query, State};
use serde::Deserialize;
use uuid::Uuid;

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
    Query(params): Query<Request>,
) -> Result<String, AppError> {
    let response = state
        .engine_client
        .cancel(CancelRequest {
            order_id: params.order_id.to_string(),
        })
        .await?
        .into_inner();
    Ok(format!("canceled {response:?}"))
}
