use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, TypedHeader},
    response::Response,
    RequestPartsExt,
};
use http::{request::Parts, StatusCode};
use redis::{AsyncCommands, Expiry};
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
pub struct User {
    id: u128,
}

#[async_trait]
impl<S> FromRequestParts<S> for User
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

        let id: u128 = store
            .get_ex(session_id, Expiry::EX(3600))
            .await
            .map_err(|e| {
                Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body(e.to_string())
                    .unwrap()
            })?;

        Ok(User { id })
    }
}
