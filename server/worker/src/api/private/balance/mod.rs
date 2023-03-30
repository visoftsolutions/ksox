mod sse;

use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};
use database::{projections::spot::valut::Valut, sqlx::types::Uuid};
use serde::Deserialize;

use crate::{
    api::{auth::models::UserId, AppError},
    models::AppState,
};

pub fn router(app_state: &AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/sse", get(sse::root))
        .with_state(app_state.clone())
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub asset_id: Uuid,
}

pub async fn root(
    State(state): State<AppState>,
    user_id: UserId,
    Query(params): Query<Request>,
) -> Result<Json<Valut>, AppError> {
    let valut = state
        .valuts_manager
        .get_or_create_for_user_asset(*user_id, params.asset_id)
        .await?;
    Ok(Json(valut))
}
