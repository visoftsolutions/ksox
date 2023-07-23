use ethereum_types::U256;
use fraction::Fraction;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;
use std::str::FromStr;
use thiserror::Error;
use tonic::Status;

use crate::{
    contracts::treasury::WithdrawFilter,
    database::managers::{assets::AssetsManager, users::UsersManager}, base,
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
