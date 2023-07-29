use std::{io, ops::Deref, str::FromStr};

use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::{PgArgumentBuffer, PgValueRef},
    Decode, Encode, Postgres, Type, TypeInfo,
};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash, Default)]
pub struct Address(pub ethereum_types::Address);

impl FromStr for Address {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes: [u8; 20] = prefix_hex::decode(s)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
        Ok(Self(ethereum_types::Address::from_slice(&bytes)))
    }
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", prefix_hex::encode(self.as_bytes()))
    }
}

impl From<Address> for ethereum_types::H160 {
    fn from(val: Address) -> Self {
        val.0
    }
}

impl From<ethereum_types::H160> for Address {
    fn from(val: ethereum_types::H160) -> Self {
        Self(val)
    }
}

impl Deref for Address {
    type Target = ethereum_types::Address;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Type<Postgres> for Address {
    fn type_info() -> <Postgres as sqlx::Database>::TypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("CHAR[]")
    }
    fn compatible(ty: &<Postgres as sqlx::Database>::TypeInfo) -> bool {
        ty.name() == "CHAR[]"
    }
}

impl Decode<'_, Postgres> for Address {
    fn decode(value: PgValueRef) -> std::result::Result<Self, sqlx::error::BoxDynError> {
        let bytes: [u8; 20] =
            prefix_hex::decode(value.as_str()?).map_err(|error| error.to_string())?;
        Ok(Address(ethereum_types::Address::from_slice(&bytes)))
    }
}

impl Encode<'_, Postgres> for Address {
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
