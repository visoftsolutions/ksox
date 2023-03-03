use axum::extract::State;

use crate::{api::auth::models::UserId, models::AppState};

pub async fn root(State(_state): State<AppState>, user_id: UserId) -> String {
    format!("mint endpoint, Hello {user_id}")
}
