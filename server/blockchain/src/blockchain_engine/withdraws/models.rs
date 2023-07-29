use std::str::FromStr;

use evm::address::Address;
use fraction::Fraction;
use thiserror::Error;
use tonic::Status;

use crate::{base, engine_base};

impl From<engine_base::TransferResponse> for engine_base::RevertTransferRequest {
    fn from(val: engine_base::TransferResponse) -> Self {
        engine_base::RevertTransferRequest { id: val.id }
    }
}

pub struct WithdrawRequest {
    pub maker_address: Address,
    pub taker_address: Address,
    pub asset_address: Address,
    pub amount: Fraction,
}

impl TryFrom<base::WithdrawRequest> for WithdrawRequest {
    type Error = Status;
    fn try_from(value: base::WithdrawRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            maker_address: Address::from_str(&value.maker_address)
                .map_err(|e| Status::invalid_argument(e.to_string()))?,
            taker_address: Address::from_str(&value.taker_address)
                .map_err(|e| Status::invalid_argument(e.to_string()))?,
            asset_address: Address::from_str(&value.asset_address)
                .map_err(|e| Status::invalid_argument(e.to_string()))?,
            amount: serde_json::from_str(&value.amount)
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
