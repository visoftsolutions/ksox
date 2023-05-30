use axum::extract::FromRef;
use database::managers::{
    assets::{AssetsManager, AssetsNotificationManager},
    candlesticks::{CandlesticksManager, CandlesticksNotificationManager},
    orders::{OrdersManager, OrdersNotificationManager},
    trades::{TradesManager, TradesNotificationManager},
    users::UsersManager,
    valuts::{ValutsManager, ValutsNotificationManager},
};
use engine_base::engine_client::EngineClient;
use fraction::Fraction;
use sqlx::PgPool;
use tonic::transport::Channel;

use crate::{
    database::{
        self,
        managers::transfers::{TransfersManager, TransfersNotificationManager},
    },
    engine_base,
    recognition::{asset_pair::AssetPairRecognition, user::UserRecognition},
};

#[derive(Clone)]
pub struct AppState {
    pub accuracy: Fraction,
    pub database: PgPool,
    pub session_store: redis::Client,
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
    pub transfers_manager: TransfersManager,
    pub transfers_notification_manager: TransfersNotificationManager,
    pub assets_pair_recognition: AssetPairRecognition,
    pub user_recognition: UserRecognition,
    pub engine_client: EngineClient<Channel>,
}

impl FromRef<AppState> for redis::Client {
    fn from_ref(state: &AppState) -> Self {
        state.session_store.clone()
    }
}

impl FromRef<AppState> for UsersManager {
    fn from_ref(state: &AppState) -> Self {
        state.users_manager.clone()
    }
}
