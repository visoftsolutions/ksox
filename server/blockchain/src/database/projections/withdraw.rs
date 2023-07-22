use chrono::{DateTime, Utc};
use evm::txhash::TxHash;
use fraction::{num_traits::One, Fraction};
use serde::{Deserialize, Serialize};
use tonic::Status;
use uuid::Uuid;

use super::Confirmable;
use crate::engine_base;
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Withdraw {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub last_modification_at: DateTime<Utc>,
    pub user_id: Uuid,
    pub asset_id: Uuid,
    pub tx_hash: TxHash,
    pub amount: Fraction,
    pub confirmations: Fraction,
}

impl Confirmable for Withdraw {
    fn set(&mut self, confirmations: usize) {
        self.confirmations = Fraction::from(confirmations) / Fraction::from(10)
    }
    fn is_confirmed(&self) -> bool {
        self.confirmations >= Fraction::one()
    }
}

impl TryInto<engine_base::TransferRequest> for Withdraw {
    type Error = Status;
    fn try_into(self) -> Result<engine_base::TransferRequest, Self::Error> {
        Ok(engine_base::TransferRequest {
            maker_id: self.user_id.to_string(),
            taker_id: Uuid::from_bytes([0x00; 16]).to_string(),
            asset_id: self.asset_id.to_string(),
            amount: serde_json::to_string(&self.amount)
                .map_err(|e| Status::invalid_argument(e.to_string()))?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WithdrawInsert {
    pub user_id: Uuid,
    pub asset_id: Uuid,
    pub tx_hash: TxHash,
    pub amount: Fraction,
    pub confirmations: Fraction,
}
