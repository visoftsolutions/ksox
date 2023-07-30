use chrono::{DateTime, Utc};
use ethereum_types::U256;
use ethers::prelude::LogMeta;
use evm::{address::Address, txhash::TxHash};
use fraction::{num_traits::One, Fraction};
use num_bigint::BigInt;
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

use super::Confirmable;
use crate::{
    blockchain_engine::models::BlockchainEngineError,
    contracts::treasury::DepositFilter,
    database::managers::{assets::AssetsManager, users::UsersManager},
    engine_base,
};
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Deposit {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub last_modification_at: DateTime<Utc>,
    pub maker_address: Address,
    pub taker_address: Address,
    pub asset_address: Address,
    pub tx_hash: TxHash,
    pub amount: Fraction,
    pub confirmations: Fraction,
}

impl Confirmable for Deposit {
    fn set(&mut self, confirmations: usize) {
        self.confirmations =
            Fraction::from_raw((BigInt::from(confirmations), BigInt::from(10))).unwrap_or_default()
    }
    fn is_confirmed(&self) -> bool {
        self.confirmations >= Fraction::one()
    }
}

pub async fn deposit_to_transfer_request(
    pool: &mut Transaction<'_, Postgres>,
    deposit: Deposit,
) -> Result<engine_base::TransferRequest, BlockchainEngineError> {
    let taker_id = UsersManager::get_or_create_by_address(pool, deposit.taker_address)
        .await?
        .id;
    let asset_id = AssetsManager::get_by_address(pool, deposit.asset_address)
        .await?
        .id;
    Ok::<engine_base::TransferRequest, BlockchainEngineError>(engine_base::TransferRequest {
        maker_id: Uuid::default().to_string(),
        taker_id: taker_id.to_string(),
        asset_id: asset_id.to_string(),
        amount: serde_json::to_string(&deposit.amount)?,
    })
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DepositInsert {
    pub maker_address: Address,
    pub taker_address: Address,
    pub asset_address: Address,
    pub tx_hash: TxHash,
    pub amount: Fraction,
    pub confirmations: Fraction,
}

impl DepositInsert {
    pub async fn from_filter<'t, 'p>(
        t: &'t mut Transaction<'p, Postgres>,
        filter: &DepositFilter,
        meta: &LogMeta,
    ) -> sqlx::Result<Self> {
        let asset = AssetsManager::get_by_address(t, filter.token_address.into()).await?;
        let mut bytes = [0_u8; 32];
        filter.amount.to_little_endian(&mut bytes);
        Ok(Self {
            maker_address: meta.address.into(),
            taker_address: filter.user_address.into(),
            asset_address: filter.token_address.into(),
            tx_hash: meta.transaction_hash.into(),
            amount: Fraction::from_bytes_le(&bytes) / asset.decimals,
            confirmations: Fraction::default(),
        })
    }

    pub async fn to_filter<'t, 'p>(
        self,
        t: &'t mut Transaction<'p, Postgres>,
    ) -> sqlx::Result<DepositFilter> {
        let asset = AssetsManager::get_by_address(t, self.asset_address.clone()).await?;
        Ok(DepositFilter {
            user_address: *self.taker_address,
            token_address: *self.asset_address,
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
