use axum::extract::FromRef;

#[derive(Clone)]
pub struct AppState {
    pub session_store: redis::Client,
}

impl FromRef<AppState> for redis::Client {
    fn from_ref(state: &AppState) -> Self {
        state.session_store.clone()
    }
}
