use std::ops::Deref;

use ethereum_types::Address;
use sqlx::{
    postgres::{PgArgumentBuffer, PgValueRef, PgTypeInfo},
    Decode, Encode, Postgres, Type
};

#[derive(Debug)]
pub struct EvmAddress(pub Address);

impl Deref for EvmAddress {
    type Target = Address;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ToString for EvmAddress {
    fn to_string(&self) -> String {
        prefix_hex::encode(self.0.as_bytes())
    }
}

impl TryFrom<String> for EvmAddress {
    type Error = prefix_hex::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let bytes = prefix_hex::decode::<[u8; 20]>(value.as_str())?;
        Ok(EvmAddress(Address::from_slice(&bytes)))
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

impl Type<Postgres> for EvmAddress {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("evm_address")
    }
}