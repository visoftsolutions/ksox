use fraction::Fraction;
use std::str::FromStr;
use thiserror::Error;
use tonic::Status;
use uuid::Uuid;

use crate::{base, blockchain_engine::withdraws::models::WithdrawEvent};

pub struct WithdrawRequest {
    pub user_id: Uuid,
    pub asset_id: Uuid,
    pub amount: Fraction,
}

impl TryFrom<base::WithdrawRequest> for WithdrawRequest {
    type Error = Status;
    fn try_from(value: base::WithdrawRequest) -> Result<Self, Self::Error> {
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

impl Into<WithdrawEvent> for WithdrawRequest {
    fn into(self) -> WithdrawEvent {
        WithdrawEvent {
            user_id: self.user_id,
            asset_id: self.asset_id,
            amount: self.amount,
        }
    }
}

impl TryInto<base::WithdrawRequest> for WithdrawRequest {
    type Error = Status;
    fn try_into(self) -> Result<base::WithdrawRequest, Self::Error> {
        Ok(base::WithdrawRequest {
            user_id: self.user_id.to_string(),
            asset_id: self.asset_id.to_string(),
            amount: serde_json::to_string(&self.amount)
                .map_err(|e| Status::invalid_argument(e.to_string()))?,
        })
    }
}

pub struct WithdrawResponse {}

impl TryFrom<Result<WithdrawResponse, WithdrawError>> for base::WithdrawResponse {
    type Error = Status;
    fn try_from(value: Result<WithdrawResponse, WithdrawError>) -> Result<Self, Self::Error> {
        let _v = value.map_err(|e| Status::aborted(e.to_string()))?;
        Ok(Self {})
    }
}

#[derive(Error, Debug)]
pub enum WithdrawError {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}
