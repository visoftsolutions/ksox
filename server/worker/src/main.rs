mod api;
mod database;
mod models;
mod ohlcv;
mod recognition;
mod shutdown_signal;

pub mod engine_base {
    tonic::include_proto!("server.engine.base");
}

pub mod blockchain_base {
    tonic::include_proto!("server.blockchain.base");
}

use axum::{routing::get, Router};
use blockchain_base::blockchain_client::BlockchainClient;
use engine_base::engine_client::EngineClient;
use evm::address::Address;
use regex::Regex;
use sqlx::postgres::PgPoolOptions;
use std::{net::SocketAddr, str::FromStr};

use crate::{
    database::managers::{
        assets::{AssetsManager, AssetsNotificationManager},
        badges::{BadgesManager, BadgesNotificationManager},
        candlesticks::{CandlesticksManager, CandlesticksNotificationManager},
        deposits::{DepositsManager, DepositsNotificationManager},
        notifications::NotificationManager,
        orders::{OrdersManager, OrdersNotificationManager},
        trades::{TradesManager, TradesNotificationManager},
        transfers::{TransfersManager, TransfersNotificationManager},
        users::{UsersManager, UsersNotificationManager},
        valuts::{ValutsManager, ValutsNotificationManager},
        withdraws::{WithdrawsManager, WithdrawsNotificationManager},
    },
    models::AppState,
    recognition::{asset_pair::AssetPairRecognition, user::UserRecognition},
};

use macros::retry;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let database = retry!(PgPoolOptions::new()
        .max_connections(10)
        .connect(std::env::var("KSOX_POSTGRES_URL").unwrap().as_str()))?;

    let notification_manager_controller =
        NotificationManager::start(database.clone(), "worker").await?;

    let app_state = AppState {
        contract_address: Address::from_str(std::env::var("CONTRACT_ADDRESS").unwrap().as_str())
            .unwrap(),
        database: database.clone(),
        session_store: redis::Client::open(
            std::env::var("KSOX_REDIS_URL").unwrap_or_default().as_str(),
        )?,
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
        deposits_manager: DepositsManager::new(database.clone()),
        deposits_notification_manager: DepositsNotificationManager::new(
            notification_manager_controller.get_subscriber(),
        ),
        withdraws_manager: WithdrawsManager::new(database.clone()),
        withdraws_notification_manager: WithdrawsNotificationManager::new(
            notification_manager_controller.get_subscriber(),
        ),
        assets_pair_recognition: AssetPairRecognition::new(
            database.clone(),
            Regex::new(r"[^a-zA-Z]+")?,
        ),
        user_recognition: UserRecognition::new(database),
        engine_client: retry!(EngineClient::connect(
            std::env::var("KSOX_SERVER_ENGINE_URL").unwrap()
        ))?,
        blockchain_client: retry!(BlockchainClient::connect(
            std::env::var("KSOX_SERVER_BLOCKCHAIN_URL").unwrap()
        ))?,
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
