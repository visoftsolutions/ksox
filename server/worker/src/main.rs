#![allow(dead_code)]

mod api;
mod cache;
mod database;
mod models;
mod ohlcv;
mod recognition;
mod shutdown_signal;

pub mod engine_base {
    tonic::include_proto!("server.engine.base");
}

use std::net::SocketAddr;

use axum::{routing::get, Router};
use engine_base::engine_client::EngineClient;
use regex::Regex;
use sqlx::postgres::PgPoolOptions;

use crate::{
    cache::get_client,
    database::managers::{
        assets::{AssetsManager, AssetsNotificationManager},
        badges::{BadgesManager, BadgesNotificationManager},
        candlesticks::{CandlesticksManager, CandlesticksNotificationManager},
        notifications::NotificationManager,
        orders::{OrdersManager, OrdersNotificationManager},
        trades::{TradesManager, TradesNotificationManager},
        transfers::{TransfersManager, TransfersNotificationManager},
        users::{UsersManager, UsersNotificationManager},
        valuts::{ValutsManager, ValutsNotificationManager},
    },
    models::AppState,
    recognition::{asset_pair::AssetPairRecognition, user::UserRecognition},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let database = PgPoolOptions::new()
        .max_connections(10)
        .connect(std::env::var("DATABASE_URL")?.as_str())
        .await?;

    let notification_manager_controller =
        NotificationManager::start(database.clone(), "notifications").await?;

    let app_state = AppState {
        accuracy: std::env::var("WORKER_FRACTION_ACCURACY")?.parse()?,
        database: database.clone(),
        session_store: get_client()?,
        users_manager: UsersManager::new(database.clone()),
        users_notification_manager: UsersNotificationManager::new(
            notification_manager_controller.get_subscriber(),
        ),
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
        transfers_manager: TransfersManager::new(database.clone()),
        transfers_notification_manager: TransfersNotificationManager::new(
            notification_manager_controller.get_subscriber(),
        ),
        badges_manager: BadgesManager::new(database.clone()),
        badges_notification_manager: BadgesNotificationManager::new(
            notification_manager_controller.get_subscriber(),
        ),
        assets_pair_recognition: AssetPairRecognition::new(
            database.clone(),
            Regex::new(r"[^a-zA-Z]+")?,
        ),
        user_recognition: UserRecognition::new(database),
        engine_client: EngineClient::connect(std::env::var("ENGINE_URL")?).await?,
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
