pub mod deposit;
pub mod withdraw;

use std::{io, ops::Deref, str::FromStr};

use ethereum_types::Secret;
use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::{PgArgumentBuffer, PgValueRef},
    Decode, Encode, Postgres,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TxAddress(pub Secret);

impl FromStr for TxAddress {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes: [u8; 32] = prefix_hex::decode(s)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
        Ok(Self(Secret::from_slice(&bytes)))
    }
}

impl std::fmt::Display for TxAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", prefix_hex::encode(self.as_bytes()))
    }
}

impl From<TxAddress> for ethereum_types::H256 {
    fn from(val: TxAddress) -> Self {
        val.0
    }
}

impl Deref for TxAddress {
    type Target = Secret;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Decode<'_, Postgres> for TxAddress {
    fn decode(value: PgValueRef) -> std::result::Result<Self, sqlx::error::BoxDynError> {
        let bytes: [u8; 20] =
            prefix_hex::decode(value.as_str()?).map_err(|error| error.to_string())?;
        Ok(TxAddress(Secret::from_slice(&bytes)))
    }
}

impl Encode<'_, Postgres> for TxAddress {
    fn produces(&self) -> Option<<Postgres as sqlx::Database>::TypeInfo> {
        Some(sqlx::postgres::PgTypeInfo::with_name("CHAR[]"))
    }

    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> sqlx::encode::IsNull {
        <&str as Encode<Postgres>>::encode_by_ref(&self.to_string().as_str(), buf)
    }

    fn encode(
        self,
        buf: &mut <Postgres as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull
    where
        Self: Sized,
    {
        self.encode_by_ref(buf)
    }
}

use chrono::{DateTime, Utc};
use fraction::Fraction;
use uuid::Uuid;

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