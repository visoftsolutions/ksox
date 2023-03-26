pub mod active;
pub mod balance;
pub mod burn;
pub mod cancel;
pub mod mint;
pub mod orders;
pub mod submit;
pub mod trades;

use axum::{
    routing::{delete, post},
    Router,
};

use crate::models::AppState;

pub fn router(app_state: &AppState) -> Router {
    Router::new()
        .route("/mint", post(mint::root))
        .route("/burn", post(burn::root))
        .route("/cancel", delete(cancel::root))
        .route("/submit", post(submit::root))
        .with_state(app_state.clone())
        .nest("/active", active::router(app_state))
        .nest("/balance", balance::router(app_state))
        .nest("/orders", orders::router(app_state))
        .nest("/trades", trades::router(app_state))
}
