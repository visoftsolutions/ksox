use std::{ops::Deref, str::FromStr};

use ethereum_types::Address;
use sqlx::{
    postgres::{PgArgumentBuffer, PgValueRef},
    Decode, Encode, Postgres,
};

#[derive(Debug, Clone)]
pub struct EvmAddress(pub Address);

impl Deref for EvmAddress {
    type Target = Address;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for EvmAddress {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = prefix_hex::decode::<[u8; 20]>(s).map_err(anyhow::Error::msg)?;
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

impl Decode<'_, Postgres> for EvmAddress {
    fn decode(value: PgValueRef) -> std::result::Result<Self, sqlx::error::BoxDynError> {
        let bytes =
            prefix_hex::decode::<[u8; 20]>(value.as_str()?).map_err(|error| error.to_string())?;
        Ok(EvmAddress(Address::from_slice(&bytes)))
    }
}

impl Encode<'_, Postgres> for EvmAddress {
    fn produces(&self) -> Option<<Postgres as sqlx::Database>::TypeInfo> {
        <&str as Encode<Postgres>>::produces(&self.to_string().as_str())
    }

    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> sqlx::encode::IsNull {
        <&str as Encode<Postgres>>::encode_by_ref(&self.to_string().as_str(), buf)
    }
}
