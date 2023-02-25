pub mod active_orders;
pub mod balance;
pub mod burn;
pub mod cancel_order;
pub mod mint;
pub mod orders;
pub mod submit_request;
pub mod trades;

use axum::{routing::get, Router};

use crate::models::AppState;

pub fn router(app_state: &AppState) -> Router {
    Router::new()
        .route("/mint", get(mint::root))
        .route("/burn", get(burn::root))
        .route("/orders", get(orders::root))
        .route("/trades", get(trades::root))
        .route("/balance", get(balance::root))
        .route("/active_orders", get(active_orders::root))
        .route("/cancel_order", get(cancel_order::root))
        .route("/submit_request", get(submit_request::root))
        .with_state(app_state.clone())
}
