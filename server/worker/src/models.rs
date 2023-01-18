use axum::extract::FromRef;
use cache::redis::Client;
use database::managers::users::UsersManager;

#[derive(Clone)]
pub struct AppState {
    pub session_store: Client,
    pub users_manager: UsersManager,
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
