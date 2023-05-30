pub mod search_user;

use axum::{routing::get, Router};

use crate::models::AppState;

pub fn router(app_state: &AppState) -> Router {
    Router::new()
        .route("/search_user", get(search_user::root))
        .with_state(app_state.clone())
}
