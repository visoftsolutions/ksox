use std::hash::Hash;

use chrono::{DateTime, Utc};
use fraction::Fraction;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use worker::database::projections::user::EvmAddress;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Valut {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub last_modification_at: DateTime<Utc>,
    pub user_address: EvmAddress,
    pub asset_address: EvmAddress,
    pub decimals: Fraction,
    pub balance: Fraction,
}
