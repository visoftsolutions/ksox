use axum::extract::FromRef;
use cache::redis::Client;
use database::managers::{
    spot::{
        assets::AssetsManager, orders::OrdersManager, trades::TradesManager, valuts::ValutsManager,
    },
    users::UsersManager,
};

#[derive(Clone)]
pub struct AppState {
    pub session_store: Client,
    pub users_manager: UsersManager,
    pub assets_manager: AssetsManager,
    pub valuts_manager: ValutsManager,
    pub trades_manager: TradesManager,
    pub orders_manager: OrdersManager,
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
