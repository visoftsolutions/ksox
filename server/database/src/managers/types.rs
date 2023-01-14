use std::ops::Deref;

use ethereum_types::Address as EthereumTypesAddress;
use sqlx::{Type, Postgres, postgres::PgTypeInfo};

pub struct Address(EthereumTypesAddress);

impl Deref for Address {
    type Target = EthereumTypesAddress;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Type<Postgres> for Address {
    fn type_info() -> <Postgres as sqlx::Database>::TypeInfo {
        "CHAR(42)"
    }
}