use std::str::FromStr;

use chrono::{DateTime, Utc};
use ethers::types::Signature;
use evm::address::Address;
use fraction::Fraction;
use thiserror::Error;
use tonic::Status;

use crate::base;

#[derive(Debug, Clone)]
pub struct WithdrawPermitRequest {
    pub spender: Address,
    pub asset: Address,
    pub amount: Fraction,
    pub deadline: DateTime<Utc>,
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
    pub signature: Signature,
}

impl From<WithdrawPermitResponse> for base::WithdrawPermitResponse {
    fn from(value: WithdrawPermitResponse) -> Self {
        Self {
            signature: prefix_hex::encode(<[u8; 65]>::from(value.signature)),
        }
    }
}

#[derive(Error, Debug)]
pub enum WithdrawPermitError {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}

#[derive(Error, Debug)]
pub enum BlockchainEngineError {
    #[error(transparent)]
    SendError(#[from] tonic::Status),

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}
