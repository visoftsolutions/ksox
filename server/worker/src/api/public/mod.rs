pub mod assets;
pub mod depth;
pub mod ohlc;
pub mod regex_search;
pub mod trades;

use crate::models::AppState;
use axum::{routing::get, Router};

pub fn router(app_state: &AppState) -> Router {
    Router::new()
        .route("/assets", get(assets::root))
        .route("/depth", get(depth::root))
        .route("/ohlc", get(ohlc::root))
        .route("/regex_search", get(regex_search::root))
        .route("/trades", get(trades::root))
        .with_state(app_state.clone())
}
