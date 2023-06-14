use axum::Router;

use crate::models::AppState;
pub mod open;
pub mod received;

pub fn router(app_state: &AppState) -> Router {
    Router::new()
        .with_state(app_state.clone())
        .nest("/open", open::router(app_state))
        .nest("/received", received::router(app_state))
}
