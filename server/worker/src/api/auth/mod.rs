pub mod models;

use super::AppError;
use crate::models::AppState;
use axum::{extract::Json, extract::State, routing::get, routing::post, Router};
use database::{managers::users::UsersManager, projections::user::User};
use futures::StreamExt;
use models::*;
use redis::AsyncCommands;
use std::ops::Deref;

pub fn router(app_state: &AppState) -> Router {
    Router::new()
        .route("/", get(generate_nonce))
        .route("/", post(validate_signature))
        .with_state(app_state.clone())
}

async fn generate_nonce(
    State(state): State<AppState>,
    Json(payload): Json<GenerateNonceRequest>,
) -> Result<Json<GenerateNonceResponse>, AppError> {
    let nonce = Nonce::new(32);
    state
        .session_store
        .get_async_connection()
        .await?
        .set_ex(
            format!("auth:nonce:{:02X?}", payload.address),
            nonce.clone(),
            NONCE_EXPIRATION_TIME,
        )
        .await?;
    Ok(Json(GenerateNonceResponse {
        nonce,
        expiration: NONCE_EXPIRATION_TIME,
    }))
}

async fn validate_signature(
    State(session_store): State<redis::Client>,
    State(users_manager): State<UsersManager>,
    Json(payload): Json<ValidateSignatureRequest>,
) -> Result<Json<ValidateSignatureResponse>, AppError> {
    let mut redis_conn = session_store.get_async_connection().await?;

    let nonce = redis_conn
        .get_del::<String, Nonce>(format!("auth:nonce:{:02X?}", payload.address.clone()))
        .await?;

    payload
        .signature
        .verify(nonce.deref().as_slice(), payload.address.clone())?;

    let session_id = SessionId::new(32);

    let mut users_stream = users_manager
        .get_by_evm_address(payload.address.clone())
        .await;
    let user = users_stream.next().await;
    if users_stream.next().await.is_some() {
        return Err(AppError(anyhow::Error::msg(
            "Multple users with same EvmAddresses",
        )));
    }

    let db_user: User = match user {
        Some(user) => user?,
        None => users_manager.insert(payload.address).await?,
    };

    redis_conn
        .set_ex(
            format!("auth:session_id:{session_id}"),
            format!("{}", db_user.id),
            SESSION_EXPIRATION_TIME,
        )
        .await?;
    Ok(Json(ValidateSignatureResponse {
        session_id,
        expiration: SESSION_EXPIRATION_TIME,
    }))
}
