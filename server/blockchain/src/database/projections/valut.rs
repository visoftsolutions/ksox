use std::hash::Hash;

use chrono::{DateTime, Utc};
use evm::address::Address;
use fraction::Fraction;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use value::Value;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Valut {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub last_modification_at: DateTime<Utc>,
    pub user_address: Address,
    pub asset_address: Address,
    pub decimals: Fraction,
    pub balance: Value,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ValutFinite {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub last_modification_at: DateTime<Utc>,
    pub user_address: Address,
    pub asset_address: Address,
    pub decimals: Fraction,
    pub balance: Fraction,
}

impl Hash for ValutFinite {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl ValutFinite {
    pub fn checked_from(valut: Valut) -> Option<Self> {
        Some(ValutFinite {
            id: valut.id,
            created_at: valut.created_at,
            last_modification_at: valut.last_modification_at,
            user_address: valut.user_address,
            asset_address: valut.asset_address,
            decimals: valut.decimals,
            balance: valut.balance.into_finite()?,
        })
    }
}
