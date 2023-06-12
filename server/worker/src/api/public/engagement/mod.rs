use axum::Router;

use crate::models::AppState;
pub mod badges;

pub fn router(app_state: &AppState) -> Router {
    Router::new()
        .with_state(app_state.clone())
        .nest("/badges", badges::router(app_state))
}
