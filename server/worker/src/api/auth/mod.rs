pub mod models;

use std::str::FromStr;

use axum::{
    extract::{Json, Query, State},
    http::{self, HeaderValue},
    response::IntoResponse,
    routing::{delete, get, post},
    Router, TypedHeader,
};
use hyper::HeaderMap;
use models::*;
use redis::AsyncCommands;

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
        .route("/session", get(session_info))
        .with_state(app_state.clone())
}

async fn generate_message(
    State(state): State<AppState>,
    Query(params): Query<GenerateNonceRequest>,
) -> Result<Json<GenerateNonceResponse>, AppError> {
    let nonce = Nonce::new(32);
    state
        .session_store
        .get_async_connection()
        .await?
        .set_ex(
            format!("auth:nonce:{}", params.address),
            nonce.to_owned(),
            MESSAGE_EXPIRATION_TIME,
        )
        .await?;
    Ok(Json(GenerateNonceResponse {
        message: Message::from(nonce),
        expiration: MESSAGE_EXPIRATION_TIME,
    }))
}

async fn validate_signature(
    State(state): State<AppState>,
    Json(payload): Json<ValidateSignatureRequest>,
) -> Result<impl IntoResponse, AppError> {
    let mut redis_conn = state.session_store.get_async_connection().await?;

    let nonce = redis_conn
        .get_del::<String, Nonce>(format!("auth:nonce:{}", payload.address.clone()))
        .await?;

    payload
        .signature
        .verify(Message::from(nonce).as_str(), payload.address.clone())?;

    let session_id = SessionId::new(32);

    let user = match state
        .users_manager
        .get_for_evm_address(payload.address.clone())
        .await
    {
        Ok(user) => Ok(user),
        Err(err) => match err {
            sqlx::Error::RowNotFound => {
                state
                    .users_manager
                    .insert_with_evmaddress(payload.address.clone())
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

    let mut headers = HeaderMap::new();
    headers.insert(
        http::header::SET_COOKIE,
        HeaderValue::from_str(format!("{}={}", COOKIE_NAME, session_id).as_str())?,
    );

    Ok((
        headers,
        Json(SessionResponse {
            address: payload.address,
            session_id,
            user_id: UserId::new(user.id),
            expiration: SESSION_EXPIRATION_TIME,
        }),
    ))
}

pub async fn logout(State(state): State<AppState>, user: User) -> Result<String, AppError> {
    let mut redis_conn = state.session_store.get_async_connection().await?;
    redis_conn
        .del(format!("auth:session_id:{}", user.session_id))
        .await?;
    Ok(format!("logout endpoint, Bye {}", user.user_id))
}

pub async fn session_info(
    cookies: TypedHeader<headers::Cookie>,
    State(state): State<AppState>,
) -> Result<Json<Option<SessionResponse>>, AppError> {
    let mut redis_conn = state.session_store.get_async_connection().await?;
    Ok(Json(match cookies.get(COOKIE_NAME) {
        Some(session_id) => {
            let key = format!("auth:session_id:{session_id}");
            match redis_conn.get::<'_, _, UserId>(key.to_owned()).await {
                Ok(user_id) => {
                    let user = state.users_manager.get_by_id(*user_id).await?;
                    Some(SessionResponse {
                        address: user.address,
                        session_id: SessionId::from_str(session_id)?,
                        user_id,
                        expiration: redis_conn
                            .pttl::<'_, _, usize>(key)
                            .await
                            .map(|t| t / 1000)?,
                    })
                }
                Err(_) => None,
            }
        }
        None => None,
    }))
}
