use std::ops::Deref;

use ethereum_types::Address as AddressFromEthereumTypes;
use sqlx::{postgres::PgValueRef, Decode, Postgres};

#[derive(Debug)]
pub struct EvmAddress(pub AddressFromEthereumTypes);

impl Deref for EvmAddress {
    type Target = AddressFromEthereumTypes;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Decode<'_, Postgres> for EvmAddress {
    fn decode(value: PgValueRef) -> Result<Self, sqlx::error::BoxDynError> {
        let bytes =
            prefix_hex::decode::<[u8; 20]>(value.as_str()?).map_err(|error| error.to_string())?;
        Ok(EvmAddress(AddressFromEthereumTypes::from_slice(&bytes)))
    }
}
