pub mod models;

use super::AppError;
use crate::models::AppState;
use axum::{extract::Json, extract::State, routing::get, routing::post, Router};
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
    State(redis): State<redis::Client>,
    Json(payload): Json<ValidateSignatureRequest>,
) -> Result<Json<ValidateSignatureResponse>, AppError> {
    let mut redis_conn = redis.get_async_connection().await?;

    let nonce = redis_conn
        .get_del::<String, Nonce>(format!("auth:nonce:{:02X?}", payload.address))
        .await?;
    payload
        .signature
        .verify(nonce.deref().as_slice(), payload.address)?;

    let session_id = SessionId::new(32);
    let user_id = SessionId::new(32); // TODO contact db to get user id
    redis_conn
        .set_ex(
            format!("auth:session_id:{:02X?}", session_id.deref().as_slice()),
            user_id,
            SESSION_EXPIRATION_TIME,
        )
        .await?;
    Ok(Json(ValidateSignatureResponse {
        session_id,
        expiration: SESSION_EXPIRATION_TIME,
    }))
}
