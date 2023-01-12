use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, TypedHeader},
    response::Response,
    RequestPartsExt,
};
use ethereum_types::U256;
use http::{request::Parts, StatusCode};
use redis::{AsyncCommands, ErrorKind, Expiry, FromRedisValue, RedisError};
use serde::{Deserialize, Serialize};

static COOKIE_NAME: &str = "SESSION";

#[derive(Clone)]
pub struct AppState {
    pub session_store: redis::Client,
}

impl FromRef<AppState> for redis::Client {
    fn from_ref(state: &AppState) -> Self {
        state.session_store.clone()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserId(U256);

impl FromRedisValue for UserId {
    fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
        match *v {
            redis::Value::Data(ref bytes) => Ok(UserId(U256::from_little_endian(bytes))),
            _ => Err(RedisError::from((
                ErrorKind::TypeError,
                "Response was of incompatible type",
                format!(
                    "{:?} (response was {:?})",
                    "Response not convertable to U256", v
                ),
            ))),
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for UserId
where
    redis::Client: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Response<String>;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let cookies = parts
            .extract::<TypedHeader<headers::Cookie>>()
            .await
            .map_err(|e| {
                Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body(e.to_string())
                    .unwrap()
            })?;

        let session_id = cookies
            .get(COOKIE_NAME)
            .ok_or(
                Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("no session cookie".to_string())
                    .unwrap(),
            )?
            .to_string();

        let mut store = redis::Client::from_ref(state)
            .get_async_connection()
            .await
            .map_err(|e| {
                Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(e.to_string())
                    .unwrap()
            })?;

        let id: UserId = store
            .get_ex(session_id, Expiry::EX(3600))
            .await
            .map_err(|e| {
                Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body(e.to_string())
                    .unwrap()
            })?;

        Ok(id)
    }
}
