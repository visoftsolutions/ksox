use ethereum_types::U256;
use fraction::Fraction;
use sqlx::{Postgres, Transaction};
use std::str::FromStr;
use thiserror::Error;
use tonic::Status;
use uuid::Uuid;

use crate::{
    base,
    contracts::treasury::WithdrawFilter,
    database::managers::{assets::AssetsManager, users::UsersManager},
    engine_base,
};

pub struct WithdrawEvent {
    pub user_id: Uuid,
    pub asset_id: Uuid,
    pub amount: Fraction,
}

impl WithdrawEvent {
    pub async fn from_filter<'t, 'p>(
        t: &'t mut Transaction<'p, Postgres>,
        filter: WithdrawFilter,
    ) -> sqlx::Result<Self> {
        let asset = AssetsManager::get_by_address(t, filter.token_address.into()).await?;
        let user = UsersManager::get_by_address(t, filter.user_address.into())
            .await?
            .ok_or(sqlx::Error::RowNotFound)?;
        let mut bytes = [0_u8; 32];
        filter.amount.to_little_endian(&mut bytes);
        Ok(Self {
            user_id: user.id,
            asset_id: asset.id,
            amount: Fraction::from_bytes_le(&bytes) / asset.decimals,
        })
    }

    pub async fn to_filter<'t, 'p>(
        self,
        t: &'t mut Transaction<'p, Postgres>,
    ) -> sqlx::Result<WithdrawFilter> {
        let asset = AssetsManager::get_by_id(t, self.asset_id).await?;
        let user = UsersManager::get_by_id(t, self.user_id).await?;
        Ok(WithdrawFilter {
            user_address: *user.address,
            token_address: *asset.address,
            amount: U256::from_little_endian(
                (*(self.amount * asset.decimals))
                    .to_integer()
                    .to_bytes_le()
                    .1
                    .as_slice(),
            ),
        })
    }
}

#[derive(Clone)]
pub struct WithdrawRequest {
    pub maker_id: Uuid,
    pub taker_id: Uuid,
    pub asset_id: Uuid,
    pub amount: Fraction,
}

impl TryFrom<base::WithdrawRequest> for WithdrawRequest {
    type Error = Status;
    fn try_from(value: base::WithdrawRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            maker_id: Uuid::from_str(&value.maker_id)
                .map_err(|e| Status::invalid_argument(e.to_string()))?,
            taker_id: Uuid::from_str(&value.taker_id)
                .map_err(|e| Status::invalid_argument(e.to_string()))?,
            asset_id: Uuid::from_str(&value.asset_id)
                .map_err(|e| Status::invalid_argument(e.to_string()))?,
            amount: serde_json::from_str(&value.amount)
                .map_err(|e| Status::invalid_argument(e.to_string()))?,
        })
    }
}

impl TryInto<base::WithdrawRequest> for WithdrawRequest {
    type Error = Status;
    fn try_into(self) -> Result<base::WithdrawRequest, Self::Error> {
        Ok(base::WithdrawRequest {
            maker_id: self.maker_id.to_string(),
            taker_id: self.taker_id.to_string(),
            asset_id: self.asset_id.to_string(),
            amount: serde_json::to_string(&self.amount)
                .map_err(|e| Status::invalid_argument(e.to_string()))?,
        })
    }
}

impl TryInto<engine_base::TransferRequest> for WithdrawRequest {
    type Error = Status;
    fn try_into(self) -> Result<engine_base::TransferRequest, Self::Error> {
        Ok(engine_base::TransferRequest {
            maker_id: self.maker_id.to_string(),
            taker_id: Uuid::from_bytes([0x00; 16]).to_string(),
            asset_id: self.asset_id.to_string(),
            amount: serde_json::to_string(&self.amount)
                .map_err(|e| Status::invalid_argument(e.to_string()))?,
        })
    }
}

impl Into<engine_base::RevertTransferRequest> for engine_base::TransferResponse {
    fn into(self) -> engine_base::RevertTransferRequest {
        engine_base::RevertTransferRequest { id: self.id }
    }
}

impl Into<WithdrawEvent> for WithdrawRequest {
    fn into(self) -> WithdrawEvent {
        WithdrawEvent {
            user_id: self.taker_id,
            asset_id: self.asset_id,
            amount: self.amount,
        }
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
