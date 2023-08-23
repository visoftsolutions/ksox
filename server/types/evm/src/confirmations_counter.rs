use num_bigint::BigInt;
use serde::{Deserialize, Serialize};
use sqlx::{
    database::HasValueRef, postgres::PgArgumentBuffer, types::BigDecimal, Database, Decode, Encode,
    Postgres, Type, TypeInfo,
};

#[derive(Debug, Clone, Hash, Default, Deserialize, Serialize)]
pub struct Confirmations {
    actual: BigInt,
    desired: BigInt,
}
impl Confirmations {
    pub fn new(desired: BigInt) -> Self {
        Confirmations {
            actual: BigInt::from(0),
            desired,
        }
    }

    pub fn from_raw(actual: BigInt, desired: BigInt) -> Self {
        Confirmations { actual, desired }
    }

    pub fn to_tuple_string(&self) -> String {
        format!("({},{})", self.actual, self.desired)
    }

    pub fn actual(&self) -> &BigInt {
        &self.actual
    }

    pub fn desired(&self) -> &BigInt {
        &self.desired
    }
}
impl<'r, DB: Database> Decode<'r, DB> for Confirmations
where
    (BigDecimal, BigDecimal): Decode<'r, DB>,
{
    fn decode(
        value: <DB as HasValueRef<'r>>::ValueRef,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let (actual, desired) = <(BigDecimal, BigDecimal) as Decode<DB>>::decode(value)?;

        let (actual_bigint, actual_exp) = actual.into_bigint_and_exponent();
        let (desired_bigint, desired_exp) = desired.into_bigint_and_exponent();
        let actual = actual_bigint * BigInt::from(10).pow(TryInto::try_into(-actual_exp)?);
        let desired = desired_bigint * BigInt::from(10).pow(TryInto::try_into(-desired_exp)?);
        Ok(Confirmations::from_raw(actual, desired))
    }
}
