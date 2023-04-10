#![allow(dead_code)]
#![feature(option_get_or_insert_default)]
#![feature(let_chains)]
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
        notifications::NotificationManager,
        spot::{
            assets::{AssetsManager, AssetsNotificationManager},
            candlesticks::{CandlesticksManager, CandlesticksNotificationManager},
            orders::{OrdersManager, OrdersNotificationManager},
            trades::{TradesManager, TradesNotificationManager},
            valuts::{ValutsManager, ValutsNotificationManager},
        },
        users::UsersManager,
    },
    sqlx::postgres::PgPoolOptions,
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

    let database = PgPoolOptions::new()
        .max_connections(100)
        .connect(std::env::var("DATABASE_URL")?.as_str())
        .await?;

    let notification_manager_controller =
        NotificationManager::start(database.clone(), "notifications").await?;

    let app_state = AppState {
        database: database.clone(),
        session_store: get_client()?,
        users_manager: UsersManager::new(database.clone()),
        assets_manager: AssetsManager::new(database.clone()),
        assets_notification_manager: AssetsNotificationManager::new(
            notification_manager_controller.get_subscriber(),
        ),
        valuts_manager: ValutsManager::new(database.clone()),
        valuts_notification_manager: ValutsNotificationManager::new(
            notification_manager_controller.get_subscriber(),
        ),
        trades_manager: TradesManager::new(database.clone()),
        trades_notification_manager: TradesNotificationManager::new(
            notification_manager_controller.get_subscriber(),
        ),
        orders_manager: OrdersManager::new(database.clone()),
        orders_notification_manager: OrdersNotificationManager::new(
            notification_manager_controller.get_subscriber(),
            OrdersManager::new(database.clone()),
        ),
        candlesticks_manager: CandlesticksManager::new(database.clone()),
        candlesticks_notification_manager: CandlesticksNotificationManager::new(
            notification_manager_controller.get_subscriber(),
        ),
        assets_pair_recognition: AssetPairRecognition::new(database, Regex::new(r"[^a-zA-Z]+")?),
        engine_client: EngineClient::connect(
            std::env::var("ENGINE_URL").unwrap().as_str().to_owned(),
        )
        .await?,
    };

    let app = Router::new()
        .route("/", get(api::root))
        .route("/sse", get(api::sse))
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
    notification_manager_controller.shutdown().await?;

    Ok(())
}
