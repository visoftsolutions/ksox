use axum::{extract::State, routing::get, Json, Router};
use evm::address::Address;
use serde::Serialize;

use crate::{api::AppError, models::AppState};

pub fn router(app_state: &AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .with_state(app_state.clone())
}

#[derive(Serialize, Clone)]
pub struct Response {
    pub contract_address: Address,
}
impl Response {
    pub fn new(contract_address: Address) -> Self {
        Self { contract_address }
    }
}

pub async fn root(State(state): State<AppState>) -> Result<Json<Response>, AppError> {
    Ok(Json(Response::new(state.contract_address)))
}
