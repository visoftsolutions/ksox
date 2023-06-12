use axum::Router;

use crate::models::AppState;

pub mod metadata;

pub fn router(app_state: &AppState) -> Router {
    Router::new()
        .with_state(app_state.clone())
        .nest("/metadata", metadata::router(app_state))
}
