use std::{io, ops::Deref, str::FromStr};

use chrono::{DateTime, Utc};
use ethereum_types::Address;
use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::{PgArgumentBuffer, PgRow, PgValueRef},
    types::Uuid,
    Decode, Encode, FromRow, Postgres, Row,
};

use ethereum_types::Secret;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub last_modification_at: DateTime<Utc>,
    pub address: EvmAddress,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
}

impl FromRow<'_, PgRow> for User {
    fn from_row(row: &PgRow) -> sqlx::Result<Self> {
        let evm_address =
            EvmAddress::from_str(&row.try_get::<'_, String, _>("address")?).map_err(|e| {
                sqlx::Error::Decode(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    e.to_string(),
                )))
            })?;
        Ok(User {
            id: row.try_get("id")?,
            created_at: row.try_get("created_at")?,
            last_modification_at: row.try_get("last_modification_at")?,
            address: evm_address,
            name: row.try_get("name")?,
            phone: row.try_get("phone")?,
            email: row.try_get("email")?,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct EvmAddress(pub Address);

impl FromStr for EvmAddress {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes: [u8; 20] = prefix_hex::decode(s)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
        Ok(Self(Address::from_slice(&bytes)))
    }
}

impl std::fmt::Display for EvmAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", prefix_hex::encode(self.as_bytes()))
    }
}

impl From<EvmAddress> for ethereum_types::H160 {
    fn from(val: EvmAddress) -> Self {
        val.0
    }
}

impl Deref for EvmAddress {
    type Target = Address;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Decode<'_, Postgres> for EvmAddress {
    fn decode(value: PgValueRef) -> std::result::Result<Self, sqlx::error::BoxDynError> {
        let bytes: [u8; 20] =
            prefix_hex::decode(value.as_str()?).map_err(|error| error.to_string())?;
        Ok(EvmAddress(Address::from_slice(&bytes)))
    }
}

impl Encode<'_, Postgres> for EvmAddress {
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