mod sse;

use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;

use crate::{
    api::{auth::models::UserId, AppError},
    database::projections::user::User,
    models::AppState,
};

pub fn router(app_state: &AppState) -> Router {
    Router::new()
        .route("/", get(get_metadata))
        .route("/", post(update_metadata))
        .route("/sse", get(sse::root))
        .with_state(app_state.clone())
}

pub async fn get_metadata(
    State(state): State<AppState>,
    user_id: UserId,
) -> Result<Json<User>, AppError> {
    let user = state.users_manager.get_by_id(*user_id).await?;
    Ok(Json(user))
}

#[derive(Deserialize)]
pub struct Request {
    pub name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
}

pub async fn update_metadata(
    State(state): State<AppState>,
    user_id: UserId,
    Json(params): Json<Request>,
) -> Result<Json<User>, AppError> {
    let mut user = state.users_manager.get_by_id(*user_id).await?;
    user.name = params.name;
    user.email = params.email;
    user.phone = params.phone;
    state.users_manager.update(user.clone()).await?;
    Ok(Json(user))
}
