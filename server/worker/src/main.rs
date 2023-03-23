#![allow(dead_code)]
#![feature(option_get_or_insert_default)]
mod api;
mod models;
mod ohlcv;
mod recognition;
mod shutdown_signal;

use std::net::SocketAddr;

use anyhow::{Ok, Result};
use axum::{routing::get, Router};
use cache::get_client;
use database::{
    managers::{
        spot::{
            assets::AssetsManager, candlesticks::CandlestickManager, orders::OrdersManager,
            trades::TradesManager, valuts::ValutsManager,
        },
        users::UsersManager,
    },
    sqlx::PgPool,
};
use models::AppState;
use regex::Regex;

use crate::recognition::AssetPairRecognition;

pub mod engine_base {
    tonic::include_proto!("server.engine.base");
}

use engine_base::engine_client::EngineClient;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let database =
        PgPool::connect(std::env::var("DATABASE_URL")?.as_str()).await?;

    let app_state = AppState {
        database: database.clone(),
        session_store: get_client()?,
        users_manager: UsersManager::new(database.clone()),
        assets_manager: AssetsManager::new(database.clone()),
        valuts_manager: ValutsManager::new(database.clone()),
        trades_manager: TradesManager::new(database.clone()),
        orders_manager: OrdersManager::new(database.clone()),
        candlesticks_manager: CandlestickManager::new(database.clone()),
        assets_pair_recognition: AssetPairRecognition::new(database, Regex::new(r"[^a-zA-Z]+")?),
        engine_client: EngineClient::connect(std::env::var("ENGINE_URL").unwrap().as_str().to_owned()).await?,
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
