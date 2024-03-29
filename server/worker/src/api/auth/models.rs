use std::{io, ops::Deref, str::FromStr};

use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
    response::Response,
    RequestPartsExt, TypedHeader,
};
use bytes::{Bytes, BytesMut};
use ethers_core::types::Signature;
use evm::address::Address;
use hyper::StatusCode;
use rand::RngCore;
use redis::{
    AsyncCommands, Client, Expiry, FromRedisValue, RedisError, RedisResult, RedisWrite,
    ToRedisArgs, Value,
};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use uuid::Uuid;

use super::{COOKIE_NAME, SESSION_EXPIRATION_TIME};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message(String);

impl From<Nonce> for Message {
    fn from(value: Nonce) -> Self {
        Self(format!(
            "Confirm identity by signing random data:\n{}",
            value,
        ))
    }
}

impl Deref for Message {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Message {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_owned()))
    }
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct Nonce(Bytes);

impl Nonce {
    pub fn new(size: usize) -> Self {
        let mut bytes = BytesMut::zeroed(size);
        rand::thread_rng().fill_bytes(bytes.as_mut());
        Self(bytes.into())
    }
}

impl FromStr for Nonce {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            prefix_hex::decode::<Vec<u8>>(s)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?
                .into(),
        ))
    }
}

impl std::fmt::Display for Nonce {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", prefix_hex::encode(self.0.to_vec()))
    }
}

impl Deref for Nonce {
    type Target = Bytes;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromRedisValue for Nonce {
    fn from_redis_value(v: &Value) -> RedisResult<Self> {
        match *v {
            Value::Data(ref bytes) => match String::from_utf8(bytes.clone()) {
                Ok(string) => Self::from_str(string.as_str())
                    .map_err(|e| RedisError::from(io::Error::new(io::ErrorKind::InvalidData, e))),
                Err(e) => Err(RedisError::from(io::Error::new(
                    io::ErrorKind::InvalidData,
                    e,
                ))),
            },
            _ => Err(RedisError::from(io::Error::from(io::ErrorKind::NotFound))),
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

#[derive(Debug, Clone)]
pub struct SessionId(Bytes);
impl SessionId {
    pub fn new(size: usize) -> Self {
        let mut bytes = BytesMut::zeroed(size);
        rand::thread_rng().fill_bytes(bytes.as_mut());
        Self(bytes.into())
    }
}

impl FromStr for SessionId {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            prefix_hex::decode::<Vec<u8>>(s)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?
                .into(),
        ))
    }
}

impl std::fmt::Display for SessionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", prefix_hex::encode(self.0.to_vec()))
    }
}

impl Deref for SessionId {
    type Target = Bytes;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromRedisValue for SessionId {
    fn from_redis_value(v: &Value) -> RedisResult<Self> {
        match *v {
            Value::Data(ref bytes) => match String::from_utf8(bytes.clone()) {
                Ok(string) => Self::from_str(string.as_str())
                    .map_err(|e| RedisError::from(io::Error::new(io::ErrorKind::InvalidData, e))),
                Err(e) => Err(RedisError::from(io::Error::new(
                    io::ErrorKind::InvalidData,
                    e,
                ))),
            },
            _ => Err(RedisError::from(io::Error::from(io::ErrorKind::NotFound))),
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

impl UserId {
    pub fn new(id: Uuid) -> Self {
        UserId(id)
    }
}

impl FromStr for UserId {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::from_str(s).map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, e.to_string())
        })?))
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
    fn from_redis_value(v: &Value) -> RedisResult<Self> {
        match *v {
            Value::Data(ref bytes) => match String::from_utf8(bytes.clone()) {
                Ok(string) => Self::from_str(string.as_str())
                    .map_err(|e| RedisError::from(io::Error::new(io::ErrorKind::InvalidData, e))),
                Err(e) => Err(RedisError::from(io::Error::new(
                    io::ErrorKind::InvalidData,
                    e,
                ))),
            },
            _ => Err(RedisError::from(io::Error::from(io::ErrorKind::NotFound))),
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
    Client: FromRef<S>,
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

        let mut store = Client::from_ref(state)
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
    pub address: Address,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateNonceResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub message: Message,
    pub expiration: usize,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct ValidateSignatureRequest {
    #[serde_as(as = "DisplayFromStr")]
    pub address: Address,
    #[serde_as(as = "DisplayFromStr")]
    pub signature: Signature,
}

// #[serde_as]
// #[derive(Debug, Serialize, Deserialize)]
// pub struct ValidateSignatureResponse {
//     #[serde_as(as = "DisplayFromStr")]
//     pub session_id: SessionId,
//     #[serde_as(as = "DisplayFromStr")]
//     pub user_id: UserId,
//     pub expiration: usize,
// }

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct SessionResponse {
    #[serde_as(as = "DisplayFromStr")]
    pub session_id: SessionId,
    #[serde_as(as = "DisplayFromStr")]
    pub user_id: UserId,
    #[serde_as(as = "DisplayFromStr")]
    pub address: Address,
    pub expiration: usize,
}

pub struct User {
    pub session_id: SessionId,
    pub user_id: UserId,
}

#[async_trait]
impl<S> FromRequestParts<S> for User
where
    Client: FromRef<S>,
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

        let mut store = Client::from_ref(state)
            .get_async_connection()
            .await
            .map_err(|e| {
                Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(e.to_string())
                    .unwrap()
            })?;

        let user_id: UserId = store
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

        Ok(User {
            session_id: SessionId::from_str(&session_id).map_err(|e| {
                Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(e.to_string())
                    .unwrap()
            })?,
            user_id,
        })
    }
}
