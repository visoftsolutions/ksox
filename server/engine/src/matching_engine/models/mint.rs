use std::str::FromStr;

use fraction::Fraction;
use sqlx::types::Uuid;
use thiserror::Error;
use tonic::Status;

use crate::base;

pub struct MintRequest {
    pub user_id: Uuid,
    pub asset_id: Uuid,
    pub amount: Fraction,
}

impl TryFrom<base::MintRequest> for MintRequest {
    type Error = Status;
    fn try_from(value: base::MintRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            user_id: Uuid::from_str(&value.user_id)
                .map_err(|e| Status::invalid_argument(e.to_string()))?,
            asset_id: Uuid::from_str(&value.asset_id)
                .map_err(|e| Status::invalid_argument(e.to_string()))?,
            amount: serde_json::from_str(&value.amount)
                .map_err(|e| Status::invalid_argument(e.to_string()))?,
        })
    }
}

impl TryInto<base::MintRequest> for MintRequest {
    type Error = Status;
    fn try_into(self) -> Result<base::MintRequest, Self::Error> {
        Ok(base::MintRequest {
            user_id: self.user_id.to_string(),
            asset_id: self.asset_id.to_string(),
            amount: serde_json::to_string(&self.amount)
                .map_err(|e| Status::invalid_argument(e.to_string()))?,
        })
    }
}

pub struct MintResponse {}

impl TryFrom<Result<MintResponse, MintError>> for base::MintResponse {
    type Error = Status;
    fn try_from(value: Result<MintResponse, MintError>) -> Result<Self, Self::Error> {
        let _v = value.map_err(|e| Status::aborted(e.to_string()))?;
        Ok(Self {})
    }
}

#[derive(Error, Debug)]
pub enum MintError {
    #[error("invalid amount value supplied")]
    InvalidAmount,

    #[error("operation not available yet")]
    Timeout,

    #[error("asset not found in db")]
    AssetNotFound,

    #[error("not enough balance in valut")]
    InsufficientBalance,

    #[error("add fractions failed")]
    CheckedAddFailed,

    #[error("sub fractions failed")]
    CheckedSubFailed,

    #[error("mul fractions failed")]
    CheckedMulFailed,

    #[error("div fractions failed")]
    CheckedDivFailed,

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}
