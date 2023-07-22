use chrono::{DateTime, Utc};
use evm::txhash::TxHash;
use fraction::{num_traits::One, Fraction};
use num_bigint::BigInt;
use serde::{Deserialize, Serialize};
use tonic::Status;
use uuid::Uuid;

use super::Confirmable;
use crate::engine_base;
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Deposit {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub last_modification_at: DateTime<Utc>,
    pub user_id: Uuid,
    pub asset_id: Uuid,
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

impl TryInto<engine_base::TransferRequest> for Deposit {
    type Error = Status;
    fn try_into(self) -> Result<engine_base::TransferRequest, Self::Error> {
        Ok(engine_base::TransferRequest {
            maker_id: Uuid::from_bytes([0x00; 16]).to_string(),
            taker_id: self.user_id.to_string(),
            asset_id: self.asset_id.to_string(),
            amount: serde_json::to_string(&self.amount)
                .map_err(|e| Status::invalid_argument(e.to_string()))?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DepositInsert {
    pub user_id: Uuid,
    pub asset_id: Uuid,
    pub tx_hash: TxHash,
    pub amount: Fraction,
    pub confirmations: Fraction,
}
