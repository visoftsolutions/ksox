use std::str::FromStr;

use chrono::{DateTime, Utc};
use ethers::prelude::{LogMeta, k256::elliptic_curve::rand_core::le};
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
    pub owner: Address,
    pub spender: Address,
    pub asset: Address,
    pub amount: Fraction,
    pub tx_hash: TxHash,
    pub confirmations: Fraction,
}

impl Deposit {
    pub async fn as_transfer_request<'t, 'p>(
        &self,
        t: &'t mut Transaction<'p, Postgres>,
    ) -> Result<engine_base::TransferRequest, BlockchainEngineError> {
        let taker_id = UsersManager::get_or_create_by_address(t, &self.owner)
            .await?
            .id;
        let asset_id = AssetsManager::get_by_address(t, &self.asset).await?.id;
        Ok::<engine_base::TransferRequest, BlockchainEngineError>(engine_base::TransferRequest {
            maker_id: Uuid::default().to_string(),
            taker_id: taker_id.to_string(),
            asset_id: asset_id.to_string(),
            amount: serde_json::to_string(&self.amount)?,
        })
    }
}

impl Confirmable for Deposit {
    fn set(&mut self, confirmations: usize) {
        match std::env::var("EVENT_CONFIRMATIONS") {
            Ok(string) => {
                match BigInt::from_str(&string) {
                    Ok(denom) => {
                        self.confirmations = Fraction::from_raw((BigInt::from(confirmations), denom)).unwrap_or_default()
                    },
                    Err(err) => {
                        tracing::error!("{err}");
                    }
                }
            },
            Err(err) => {
                tracing::error!("EVENT_CONFIRMATIONS env: {err}");
            }
        }
        
    }
    fn is_confirmed(&self) -> bool {
        self.confirmations >= Fraction::one()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DepositInsert {
    pub owner: Address,
    pub spender: Address,
    pub asset: Address,
    pub amount: Fraction,
    pub tx_hash: TxHash,
    pub confirmations: Fraction,
}

impl DepositInsert {
    pub async fn from_filter<'t, 'p>(
        t: &'t mut Transaction<'p, Postgres>,
        filter: &DepositFilter,
        meta: &LogMeta,
    ) -> sqlx::Result<Self> {
        let asset = AssetsManager::get_by_address(t, &Address(filter.token)).await?;
        Ok(Self {
            owner: filter.owner.into(),
            spender: filter.spender.into(),
            asset: filter.token.into(),
            tx_hash: meta.transaction_hash.into(),
            amount: Fraction::from(filter.amount) / asset.decimals,
            confirmations: Fraction::default(),
        })
    }
}
