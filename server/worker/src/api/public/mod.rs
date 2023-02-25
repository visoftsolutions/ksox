pub mod assets;
pub mod depth;
pub mod ohlc;
pub mod search;
pub mod trades;

use axum::{routing::get, Router};

use crate::models::AppState;

pub fn router(app_state: &AppState) -> Router {
    Router::new()
        .route("/ohlc", get(ohlc::root))
        .route("/depth", get(depth::root))
        .route("/assets", get(assets::root))
        .route("/search", get(search::root))
        .route("/trades", get(trades::root))
        .with_state(app_state.clone())
}
