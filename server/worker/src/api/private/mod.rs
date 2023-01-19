pub mod balance;
pub mod cancel_orders;
pub mod open_orders;
pub mod submit_orders;

use axum::{routing::get, Router};

use crate::models::AppState;

pub fn router(app_state: &AppState) -> Router {
    Router::new()
        .route("/balance", get(balance::root))
        .route("/cancel_orders", get(cancel_orders::root))
        .route("/open_orders", get(open_orders::root))
        .route("/submit_orders", get(submit_orders::root))
        .with_state(app_state.clone())
}
