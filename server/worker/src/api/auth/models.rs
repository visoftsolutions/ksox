use ethers_core::types::{Address, Signature};
use rand::RngCore;
use redis::{ErrorKind, FromRedisValue, RedisError, RedisWrite, ToRedisArgs, Value};
use serde::{Deserialize, Serialize};
use std::ops::Deref;

pub const NONCE_EXPIRATION_TIME: usize = 100; // in secodns
pub const SESSION_EXPIRATION_TIME: usize = 3600; // in secodns

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nonce(Vec<u8>);
// [u8; 32]
impl Nonce {
    pub fn new(size: usize) -> Self {
        let mut nonce = Vec::<u8>::with_capacity(size);
        rand::thread_rng().fill_bytes(nonce.as_mut_slice());
        Nonce(nonce)
    }
}

impl From<Vec<u8>> for Nonce {
    fn from(value: Vec<u8>) -> Self {
        Nonce(value)
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
            Value::Data(ref bytes) => Ok(Self::from(bytes.clone())),
            _ => Err(RedisError::from((
                ErrorKind::TypeError,
                "Response was of incompatible type",
                format!(
                    "{:?} (response was {:?})",
                    "Response type not Nonce compatible.", v
                ),
            ))),
        }
    }
}

impl ToRedisArgs for Nonce {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg(self.0.as_slice())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionId(Vec<u8>);
// [u8; 32]
impl SessionId {
    pub fn new(size: usize) -> Self {
        let mut nonce = Vec::<u8>::with_capacity(size);
        rand::thread_rng().fill_bytes(nonce.as_mut_slice());
        Self(nonce)
    }
}

impl From<Vec<u8>> for SessionId {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
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
            Value::Data(ref bytes) => Ok(Self::from(bytes.clone())),
            _ => Err(RedisError::from((
                ErrorKind::TypeError,
                "Response was of incompatible type",
                format!(
                    "{:?} (response was {:?})",
                    "Response type not SessionId compatible.", v
                ),
            ))),
        }
    }
}

impl ToRedisArgs for SessionId {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        out.write_arg(self.0.as_slice())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateNonceRequest {
    pub address: Address,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateNonceResponse {
    pub nonce: Nonce,
    pub expiration: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateSignatureRequest {
    pub address: Address,
    pub signature: Signature,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateSignatureResponse {
    pub session_id: SessionId,
    pub expiration: usize,
}
