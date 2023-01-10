mod depth;
mod trades;

use axum::{
    routing::get,
    Router,
};

pub fn router() -> Router {
    Router::new()
        .route("/depth", get(depth::depth))
        .route("/trades", get(trades::trades))
}
