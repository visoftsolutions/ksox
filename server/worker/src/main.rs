#![allow(dead_code)]
mod api;
mod models;
mod redis;
mod shutdown_signal;

use anyhow::{Ok, Result};
use axum::{routing::get, Router};
use models::AppState;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let app_state = AppState {
        session_store: redis::get_redis_client()?,
    };

    let app = Router::new()
        .route("/", get(root))
        .nest("/auth", api::auth::router(&app_state))
        .nest("/private", api::private::router(&app_state))
        .nest("/public", api::public::router(&app_state));

    let addr = SocketAddr::from(([0, 0, 0, 0], 80));
    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(async {
            shutdown_signal::listen().await;
        })
        .await?;

    Ok(())
}

async fn root() -> &'static str {
    "Hello, World!"
}
