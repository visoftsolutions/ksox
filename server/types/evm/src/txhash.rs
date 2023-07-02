use std::{io, ops::Deref, str::FromStr};

use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::{PgArgumentBuffer, PgValueRef},
    Decode, Encode, Postgres,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TxHash(pub ethereum_types::Secret);

impl FromStr for TxHash {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes: [u8; 32] = prefix_hex::decode(s)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
        Ok(Self(ethereum_types::Secret::from_slice(&bytes)))
    }
}

impl std::fmt::Display for TxHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", prefix_hex::encode(self.as_bytes()))
    }
}

impl From<TxHash> for ethereum_types::H256 {
    fn from(val: TxHash) -> Self {
        val.0
    }
}

impl From<ethereum_types::H256> for TxHash {
    fn from(val: ethereum_types::H256) -> Self {
        Self(val)
    }
}

impl Deref for TxHash {
    type Target = ethereum_types::Secret;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Decode<'_, Postgres> for TxHash {
    fn decode(value: PgValueRef) -> std::result::Result<Self, sqlx::error::BoxDynError> {
        let bytes: [u8; 20] =
            prefix_hex::decode(value.as_str()?).map_err(|error| error.to_string())?;
        Ok(TxHash(ethereum_types::Secret::from_slice(&bytes)))
    }
}

impl Encode<'_, Postgres> for TxHash {
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
