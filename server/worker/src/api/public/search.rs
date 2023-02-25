use axum::extract::State;

use crate::models::AppState;

pub async fn root(State(state): State<AppState>) -> String {
    format!("search endpoint")
}
