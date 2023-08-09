use std::str::FromStr;

use chrono::{DateTime, Utc};
use ethereum_types::{Signature, U256};
use evm::address::Address;
use fraction::Fraction;
use linked_hash_map::LinkedHashMap;
use sqlx::{Postgres, Transaction};
use std::hash::Hash;
use thiserror::Error;
use tonic::Status;
use uuid::Uuid;

use crate::{
    base,
    contracts::{treasury::WithdrawFilter, WithdrawPermit},
    database::{
        managers::assets::AssetsManager,
        projections::{withdraw::Withdraw, Expirable},
    },
    engine_base,
};

impl From<engine_base::TransferResponse> for engine_base::RevertTransferRequest {
    fn from(val: engine_base::TransferResponse) -> Self {
        engine_base::RevertTransferRequest { id: val.id }
    }
}

#[derive(Debug, Clone)]
pub struct WithdrawPermitRequest {
    pub spender: Address,
    pub asset: Address,
    pub amount: Fraction,
    pub deadline: DateTime<Utc>,
}

impl WithdrawPermitRequest {
    pub async fn to_permit<'t, 'p>(
        self,
        t: &'t mut Transaction<'p, Postgres>,
        owner: Address,
        nonce: U256,
    ) -> sqlx::Result<WithdrawPermit> {
        let asset = AssetsManager::get_by_address(t, self.asset.clone()).await?;
        Ok(WithdrawPermit {
            owner: *owner,
            spender: *self.spender,
            token: *self.asset,
            value: U256::from_little_endian(
                (*(self.amount * asset.decimals))
                    .to_integer()
                    .to_bytes_le()
                    .1
                    .as_slice(),
            ),
            nonce,
            deadline: U256::from(self.deadline.timestamp()),
        })
    }
}

impl TryFrom<base::WithdrawPermitRequest> for WithdrawPermitRequest {
    type Error = Status;
    fn try_from(value: base::WithdrawPermitRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            spender: Address::from_str(&value.spender)
                .map_err(|e| Status::invalid_argument(e.to_string()))?,
            asset: Address::from_str(&value.asset)
                .map_err(|e| Status::invalid_argument(e.to_string()))?,
            amount: serde_json::from_str(&value.amount)
                .map_err(|e| Status::invalid_argument(e.to_string()))?,
            deadline: DateTime::<Utc>::from_str(&value.deadline)
                .map_err(|e| Status::invalid_argument(e.to_string()))?,
        })
    }
}

pub struct WithdrawPermitResponse {
    signature: Signature,
}

impl TryFrom<Result<WithdrawPermitResponse, WithdrawPermitError>> for base::WithdrawPermitResponse {
    type Error = Status;
    fn try_from(
        value: Result<WithdrawPermitResponse, WithdrawPermitError>,
    ) -> Result<Self, Self::Error> {
        let v = value.map_err(|e| Status::aborted(e.to_string()))?;
        Ok(Self {
            signature: v.signature.to_string(),
        })
    }
}

#[derive(Error, Debug)]
pub enum WithdrawPermitError {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct WithdrawQueueKey {
    pub owner: Address,
    pub spender: Address,
    pub asset: Address,
    pub amount: Fraction,
    pub nonce: i64,
}

impl From<Withdraw> for WithdrawQueueKey {
    fn from(value: Withdraw) -> Self {
        Self {
            owner: value.owner,
            spender: value.spender,
            asset: value.asset,
            amount: value.amount,
            nonce: value.nonce,
        }
    }
}

impl WithdrawQueueKey {
    pub async fn from_filter<'t, 'p>(
        t: &'t mut Transaction<'p, Postgres>,
        filter: &WithdrawFilter,
    ) -> sqlx::Result<Self> {
        let asset = AssetsManager::get_by_address(t, filter.token.into()).await?;
        let mut bytes = [0_u8; 32];
        filter.amount.to_little_endian(&mut bytes);
        Ok(Self {
            owner: filter.owner.into(),
            spender: filter.spender.into(),
            asset: filter.token.into(),
            amount: Fraction::from_bytes_le(&bytes) / asset.decimals,
            nonce: filter.nonce.low_u32().into(),
        })
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct WithdrawQueueValue {
    pub deadline: DateTime<Utc>,
    pub transfer: Uuid,
}

impl Expirable for WithdrawQueueValue {
    fn is_expired(&self, time: &DateTime<Utc>) -> bool {
        time > &self.deadline
    }
}

pub struct WithdrawQueue {
    entries: LinkedHashMap<WithdrawQueueKey, WithdrawQueueValue>,
}

impl WithdrawQueue {
    pub fn new() -> Self {
        Self {
            entries: LinkedHashMap::new(),
        }
    }

    pub fn insert(&mut self, key: WithdrawQueueKey, value: WithdrawQueueValue) -> bool {
        self.entries.insert(key, value).is_some()
    }

    pub fn remove(&mut self, key: &WithdrawQueueKey) -> bool {
        self.entries.remove(key).is_some()
    }

    pub fn eval(&mut self, time: &DateTime<Utc>) -> Vec<WithdrawQueueValue> {
        let mut expired = Vec::new();
        while let Some((_, value)) = self.entries.front() {
            if value.is_expired(time) {
                if let Some((_, value)) = self.entries.pop_front() {
                    expired.push(value);
                }
            }
        }
        expired
    }
}
