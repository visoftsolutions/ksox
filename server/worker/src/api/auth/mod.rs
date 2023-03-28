pub mod models;

use std::ops::Deref;

use axum::{
    extract::{Json, Query, State},
    routing::{delete, get, post},
    Router,
};
use cache::redis::AsyncCommands;
use database::sqlx::error::Error;
use models::*;

use super::AppError;
use crate::models::AppState;

pub const MESSAGE_EXPIRATION_TIME: usize = 60; // in seconds
pub const SESSION_EXPIRATION_TIME: usize = 3600; // in seconds
pub const COOKIE_NAME: &str = "session_id";

pub fn router(app_state: &AppState) -> Router {
    Router::new()
        .route("/", get(generate_message))
        .route("/", post(validate_signature))
        .route("/", delete(logout))
        .with_state(app_state.clone())
}

async fn generate_message(
    State(state): State<AppState>,
    Query(params): Query<GenerateNonceRequest>,
) -> Result<Json<GenerateNonceResponse>, AppError> {
    let nonce = Message::new(32);
    state
        .session_store
        .get_async_connection()
        .await?
        .set_ex(
            format!("auth:nonce:{}", params.address),
            nonce.clone(),
            MESSAGE_EXPIRATION_TIME,
        )
        .await?;
    Ok(Json(GenerateNonceResponse {
        nonce,
        expiration: MESSAGE_EXPIRATION_TIME,
    }))
}

async fn validate_signature(
    State(state): State<AppState>,
    Json(payload): Json<ValidateSignatureRequest>,
) -> Result<Json<ValidateSignatureResponse>, AppError> {
    let mut redis_conn = state.session_store.get_async_connection().await?;

    let nonce = redis_conn
        .get_del::<String, Message>(format!("auth:nonce:{}", payload.address.clone()))
        .await?;

    payload
        .signature
        .verify(nonce.deref().to_owned(), payload.address.clone())?;

    let session_id = SessionId::new(32);

    let user = match state
        .users_manager
        .get_for_evm_address(payload.address.clone())
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
        user_id: UserId::new(user.id),
        expiration: SESSION_EXPIRATION_TIME,
    }))
}

pub async fn logout(State(state): State<AppState>, user: User) -> Result<String, AppError> {
    let mut redis_conn = state.session_store.get_async_connection().await?;
    redis_conn
        .del(format!("auth:session_id:{}", user.session_id))
        .await?;
    Ok(format!("logout endpoint, Bye {}", user.user_id))
}
