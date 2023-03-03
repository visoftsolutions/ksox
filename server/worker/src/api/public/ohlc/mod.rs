mod sse;

use axum::{extract::State, routing::get, Router};

use crate::{api::AppError, models::AppState};

pub fn router(app_state: &AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/sse", get(sse::root))
        .with_state(app_state.clone())
}

pub async fn root(State(_state): State<AppState>) -> Result<String, AppError> {
    Ok("hi it is ohlc endpoint".to_string())
}
