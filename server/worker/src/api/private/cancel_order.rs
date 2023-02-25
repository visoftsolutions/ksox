use axum::extract::State;

use crate::{api::auth::models::UserId, models::AppState};

pub async fn root(State(state): State<AppState>, user_id: UserId) -> String {
    format!("cancel_order endpoint, Hello {user_id}")
}
