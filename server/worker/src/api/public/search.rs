use axum::extract::State;

use crate::models::AppState;

// Send string phrase and return vector of suggestions sorted by most "relevant"
pub async fn root(State(_state): State<AppState>) -> String {
    "search endpoint".to_string()
}
