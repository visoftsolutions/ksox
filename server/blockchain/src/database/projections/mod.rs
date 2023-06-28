pub mod deposit;
pub mod valut;
pub mod withdraw;

use chrono::{DateTime, Utc};
use fraction::Fraction;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use worker::database::projections::user::TxAddress;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Flow {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub last_modification_at: DateTime<Utc>,
    pub user_id: Uuid,
    pub asset_id: Uuid,
    pub tx_hash: TxAddress,
    pub amount: Fraction,
    pub confirmations: Fraction,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FlowInsert {
    pub user_id: Uuid,
    pub asset_id: Uuid,
    pub tx_hash: TxAddress,
    pub amount: Fraction,
    pub confirmations: Fraction,
}
