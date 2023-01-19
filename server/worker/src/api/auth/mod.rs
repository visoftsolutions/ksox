pub mod models;

use std::ops::Deref;

use axum::{
    extract::{Json, State},
    routing::{get, post},
    Router,
};
use cache::redis::{AsyncCommands, Client};
use database::{managers::users::UsersManager, sqlx::error::Error};
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
    State(session_store): State<Client>,
    State(users_manager): State<UsersManager>,
    Json(payload): Json<ValidateSignatureRequest>,
) -> Result<Json<ValidateSignatureResponse>, AppError> {
    let mut redis_conn = session_store.get_async_connection().await?;

    let nonce = redis_conn
        .get_del::<String, Nonce>(format!("auth:nonce:{:02X?}", payload.address.clone()))
        .await?;

    payload
        .signature
        .verify(nonce.deref().as_ref(), payload.address.clone())?;

    let session_id = SessionId::new(32);

    let user = match users_manager
        .get_by_evm_address(payload.address.clone())
        .await
    {
        Ok(user) => Ok(user),
        Err(err) => match err {
            Error::RowNotFound => users_manager.insert(payload.address).await,
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
