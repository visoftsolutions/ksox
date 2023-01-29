pub mod models;

use std::ops::Deref;

use axum::{
    extract::{Json, State},
    routing::{get, post},
    Router,
};
use cache::redis::AsyncCommands;
use database::sqlx::error::Error;
use models::*;

use super::AppError;
use crate::models::AppState;

pub const NONCE_EXPIRATION_TIME: usize = 60; // in secodns
pub const SESSION_EXPIRATION_TIME: usize = 3600; // in secodns
pub const COOKIE_NAME: &str = "session_id";

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
            format!("auth:nonce:{}", payload.address),
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
    State(state): State<AppState>,
    Json(payload): Json<ValidateSignatureRequest>,
) -> Result<Json<ValidateSignatureResponse>, AppError> {
    let mut redis_conn = state.session_store.get_async_connection().await?;

    let nonce = redis_conn
        .get_del::<String, Nonce>(format!("auth:nonce:{}", payload.address.clone()))
        .await?;

    payload
        .signature
        .verify(nonce.deref().as_ref(), payload.address.clone())?;

    let session_id = SessionId::new(32);

    let user = match state
        .users_manager
        .get_by_evm_address(payload.address.clone())
        .await
    {
        Ok(user) => Ok(user),
        Err(err) => match err {
            Error::RowNotFound => {
                state
                    .users_manager
                    .insert_with_evmaddress(payload.address)
                    .await
            }
            _ => Err(err),
        },
    }?;

    redis_conn
        .set_ex(
            format!("auth:session_id:{session_id}"),
            format!("{}", user.id),
            SESSION_EXPIRATION_TIME,
        )
        .await?;
    Ok(Json(ValidateSignatureResponse {
        session_id,
        expiration: SESSION_EXPIRATION_TIME,
    }))
}
