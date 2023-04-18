pub mod assets;
pub mod depth;
pub mod ohlcv;
pub mod search;
pub mod trades;

use axum::{routing::get, Router};

use crate::models::AppState;

pub fn router(app_state: &AppState) -> Router {
    Router::new()
        .route("/assets", get(assets::root))
        .route("/search", get(search::root))
        .with_state(app_state.clone())
        .nest("/depth", depth::router(app_state))
        .nest("/ohlcv", ohlcv::router(app_state))
        .nest("/trades", trades::router(app_state))
}
