mod cancel;
mod submit;

use axum::{routing::post, Router};

pub fn router() -> Router {
    Router::new()
        .route("/cancel", post(cancel::cancel))
        .route("/submit", post(submit::submit))
}
