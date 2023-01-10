pub mod balance;
pub mod cancel_order;
pub mod open_orders;
pub mod submit_order;

use axum::{routing::get, Router};

pub fn router() -> Router {
    Router::new()
        .route("/balance", get(balance::root))
        .route("/cancel_order", get(cancel_order::root))
        .route("/open_orders", get(open_orders::root))
        .route("/submit_order", get(submit_order::root))
}
