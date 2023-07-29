mod sse;

use axum::{extract::State, routing::get, Json, Router};
use futures::TryStreamExt;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    api::{auth::models::UserId, AppError},
    database::projections::deposit::Deposit,
    models::AppState,
};

pub fn router(app_state: &AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/sse", get(sse::root))
        .with_state(app_state.clone())
}

#[derive(Deserialize)]
pub struct Request {
    pub asset_id: Uuid,
}

pub async fn root(
    State(state): State<AppState>,
    user_id: UserId,
) -> Result<Json<Vec<Deposit>>, AppError> {
    let user = state.users_manager.get_by_id(*user_id).await?;
    let deposits = state
        .deposits_manager
        .get_all_for_user(user.address)
        .try_collect()
        .await?;
    Ok(Json(deposits))
}
