use axum::extract::FromRef;
use cache::redis::Client;
use database::{
    managers::{
        spot::{
            assets::{AssetsManager, AssetsNotificationManager},
            candlesticks::{CandlesticksManager, CandlesticksNotificationManager},
            orders::{OrdersManager, OrdersNotificationManager},
            trades::{TradesManager, TradesNotificationManager},
            valuts::{ValutsManager, ValutsNotificationManager},
        },
        users::UsersManager,
    },
    sqlx::postgres::PgPool,
};
use tonic::transport::Channel;

use crate::{engine_base::engine_client::EngineClient, recognition::AssetPairRecognition};

#[derive(Clone)]
pub struct AppState {
    pub database: PgPool,
    pub session_store: Client,
    pub users_manager: UsersManager,
    pub assets_manager: AssetsManager,
    pub assets_notification_manager: AssetsNotificationManager,
    pub valuts_manager: ValutsManager,
    pub valuts_notification_manager: ValutsNotificationManager,
    pub trades_manager: TradesManager,
    pub trades_notification_manager: TradesNotificationManager,
    pub orders_manager: OrdersManager,
    pub orders_notification_manager: OrdersNotificationManager,
    pub candlesticks_manager: CandlesticksManager,
    pub candlesticks_notification_manager: CandlesticksNotificationManager,
    pub assets_pair_recognition: AssetPairRecognition,
    pub engine_client: EngineClient<Channel>,
}

impl FromRef<AppState> for Client {
    fn from_ref(state: &AppState) -> Self {
        state.session_store.clone()
    }
}

impl FromRef<AppState> for UsersManager {
    fn from_ref(state: &AppState) -> Self {
        state.users_manager.clone()
    }
}
