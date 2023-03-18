use axum::extract::FromRef;
use cache::redis::Client;
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
use tonic::transport::Channel;

use crate::{engine_base::engine_client::EngineClient, recognition::AssetPairRecognition};

#[derive(Clone)]
pub struct AppState {
    pub database: PgPool,
    pub session_store: Client,
    pub users_manager: UsersManager,
    pub assets_manager: AssetsManager,
    pub valuts_manager: ValutsManager,
    pub trades_manager: TradesManager,
    pub orders_manager: OrdersManager,
    pub candlesticks_manager: CandlestickManager,
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
