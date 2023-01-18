use database::types::EvmAddress;
use database::sqlx::types::Uuid;
use ethers_core::types::Signature;
use rand::RngCore;
use redis::{FromRedisValue, RedisError, RedisWrite, ToRedisArgs, Value};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use std::io::{Error, ErrorKind};
use std::{ops::Deref, str::FromStr};

use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, TypedHeader},
    response::Response,
    RequestPartsExt,
};
use http::{request::Parts, StatusCode};
use redis::{AsyncCommands, Expiry};

pub const NONCE_EXPIRATION_TIME: usize = 60; // in secodns
pub const SESSION_EXPIRATION_TIME: usize = 3600; // in secodns
static COOKIE_NAME: &str = "session_id";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nonce(Vec<u8>);

impl Nonce {
    pub fn new(size: usize) -> Self {
        let mut nonce = Vec::<u8>::new();
        nonce.resize(size, 0);
        rand::thread_rng().fill_bytes(nonce.as_mut_slice());
        Self(nonce)
    }
}

impl FromStr for Nonce {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(prefix_hex::decode(s).map_err(anyhow::Error::msg)?))
    }
}

impl std::fmt::Display for Nonce {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", prefix_hex::encode(self.0.clone()))
    }
}

impl Deref for Nonce {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromRedisValue for Nonce {
    fn from_redis_value(v: &Value) -> redis::RedisResult<Self> {
        match *v {
            Value::Data(ref bytes) => match String::from_utf8(bytes.clone()) {
                Ok(string) => Self::from_str(string.as_str())
                    .map_err(|e| RedisError::from(Error::new(ErrorKind::InvalidData, e))),
                Err(e) => Err(RedisError::from(Error::new(ErrorKind::InvalidData, e))),
            },
            _ => Err(RedisError::from(Error::from(ErrorKind::NotFound))),
        }
    }
}

impl ToRedisArgs for Nonce {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg(format!("{self}").as_bytes())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionId(Vec<u8>);
// [u8; 32]
impl SessionId {
    pub fn new(size: usize) -> Self {
        let mut nonce = Vec::<u8>::new();
        nonce.resize(size, 0);
        rand::thread_rng().fill_bytes(nonce.as_mut_slice());
        Self(nonce)
    }
}

impl From<Vec<u8>> for SessionId {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}

impl FromStr for SessionId {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(prefix_hex::decode(s).map_err(anyhow::Error::msg)?))
    }
}

impl std::fmt::Display for SessionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", prefix_hex::encode(self.0.clone()))
    }
}

impl Deref for SessionId {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromRedisValue for SessionId {
    fn from_redis_value(v: &Value) -> redis::RedisResult<Self> {
        match *v {
            Value::Data(ref bytes) => match String::from_utf8(bytes.clone()) {
                Ok(string) => Self::from_str(string.as_str())
                    .map_err(|e| RedisError::from(Error::new(ErrorKind::InvalidData, e))),
                Err(e) => Err(RedisError::from(Error::new(ErrorKind::InvalidData, e))),
            },
            _ => Err(RedisError::from(Error::from(ErrorKind::NotFound))),
        }
    }
}

impl ToRedisArgs for SessionId {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg(format!("{self}").as_bytes())
    }
}

#[derive(Debug, Clone)]
pub struct UserId(Uuid);

impl FromStr for UserId {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::from_str(s)?))
    }
}

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for UserId {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromRedisValue for UserId {
    fn from_redis_value(v: &Value) -> redis::RedisResult<Self> {
        match *v {
            Value::Data(ref bytes) => match String::from_utf8(bytes.clone()) {
                Ok(string) => Self::from_str(string.as_str())
                    .map_err(|e| RedisError::from(Error::new(ErrorKind::InvalidData, e))),
                Err(e) => Err(RedisError::from(Error::new(ErrorKind::InvalidData, e))),
            },
            _ => Err(RedisError::from(Error::from(ErrorKind::NotFound))),
        }
    }
}

impl ToRedisArgs for UserId {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg(format!("{self}").as_bytes())
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
                    .body(format!("invalid header: {e}"))
                    .unwrap()
            })?;

        let session_id = cookies
            .get(COOKIE_NAME)
            .ok_or_else(|| {
                Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body("no session cookie".to_string())
                    .unwrap()
            })?
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
            .get_ex(
                format!("auth:session_id:{session_id}"),
                Expiry::EX(SESSION_EXPIRATION_TIME),
            )
            .await
            .map_err(|e| {
                Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body(format!("session_id invalid or not found: {e}"))
                    .unwrap()
            })?;

        Ok(id)
    }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateNonceRequest {
    #[serde_as(as = "DisplayFromStr")]
    pub address: EvmAddress,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateNonceResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub nonce: Nonce,
    pub expiration: usize,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct ValidateSignatureRequest {
    #[serde_as(as = "DisplayFromStr")]
    pub address: EvmAddress,
    #[serde_as(as = "DisplayFromStr")]
    pub signature: Signature,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct ValidateSignatureResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub session_id: SessionId,
    pub expiration: usize,
}
