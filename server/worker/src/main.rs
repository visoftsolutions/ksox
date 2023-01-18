#![allow(dead_code)]
mod api;
mod models;
mod shutdown_signal;

use std::net::SocketAddr;

use anyhow::{Ok, Result};
use axum::{routing::get, Router};
use cache::get_client;
use database::{managers::users::UsersManager, sqlx::PgPool};
use models::AppState;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let database =
        PgPool::connect(std::env::var("DATABASE_URL").unwrap_or_default().as_str()).await?;

    let app_state = AppState {
        session_store: get_client()?,
        users_manager: UsersManager::new(database),
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
