use axum::extract::FromRef;
use database::managers::users::UsersManager;

#[derive(Clone)]
pub struct AppState {
    pub session_store: redis::Client,
    pub users_manager: UsersManager,
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
