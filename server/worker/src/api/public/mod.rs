pub mod assets;
pub mod depth;
pub mod ohlcv;
pub mod trades;
pub mod users;

use axum::Router;
use chrono::{DateTime, Utc};
use fraction::Fraction;
use serde::Serialize;

use super::Direction;
use crate::models::AppState;

pub fn router(app_state: &AppState) -> Router {
    Router::new()
        .with_state(app_state.clone())
        .nest("/depth", depth::router(app_state))
        .nest("/ohlcv", ohlcv::router(app_state))
        .nest("/trades", trades::router(app_state))
        .nest("/assets", assets::router(app_state))
        .nest("/users", users::router(app_state))
}

#[derive(Serialize)]
pub struct ResponseTrade {
    pub price: Fraction,
    pub volume: Fraction,
    pub time: DateTime<Utc>,
    pub direction: Direction,
}
