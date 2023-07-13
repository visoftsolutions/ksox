use ethereum_types::U256;
use fraction::Fraction;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

use crate::{
    contracts::treasury::WithdrawFilter,
    database::managers::{assets::AssetsManager, users::UsersManager},
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
