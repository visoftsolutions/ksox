use std::{
    ops::{Add, AddAssign, Deref, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
    str::FromStr,
};

use bigdecimal::{num_traits::CheckedDiv, BigDecimal};
use num_bigint::{BigInt, ToBigInt};
use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::{PgArgumentBuffer, PgValueRef},
    Decode, Encode, Postgres,
};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct Volume(BigInt);

impl Deref for Volume {
    type Target = BigInt;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Volume {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(BigInt::from_str(&s)?))
    }
}

impl std::fmt::Display for Volume {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<BigDecimal> for Volume {
    fn from(value: BigDecimal) -> Self {
        Volume(value.to_bigint().unwrap())
    }
}

impl From<BigInt> for Volume {
    fn from(value: BigInt) -> Self {
        Volume(value)
    }
}

impl Into<BigDecimal> for Volume {
    fn into(self) -> BigDecimal {
        BigDecimal::from(self.0)
    }
}

impl Decode<'_, Postgres> for Volume {
    fn decode(value: PgValueRef) -> std::result::Result<Self, sqlx::error::BoxDynError> {
        Ok(Volume(
            <BigDecimal as Decode<Postgres>>::decode(value)?
                .as_bigint_and_exponent()
                .0,
        ))
    }
}

impl Encode<'_, Postgres> for Volume {
    fn produces(&self) -> Option<<Postgres as sqlx::Database>::TypeInfo> {
        <BigDecimal as Encode<Postgres>>::produces(&BigDecimal::from(self.0.to_owned()))
    }

    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> sqlx::encode::IsNull {
        <BigDecimal as Encode<Postgres>>::encode_by_ref(&BigDecimal::from(self.0.to_owned()), buf)
    }
}

impl Add for Volume {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Volume(self.0 + rhs.0)
    }
}

impl AddAssign for Volume {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Sub for Volume {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Volume(self.0 - rhs.0)
    }
}

impl SubAssign for Volume {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl Mul for Volume {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Volume(self.0 * rhs.0)
    }
}

impl MulAssign for Volume {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}

impl Div for Volume {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Volume(self.0 / rhs.0)
    }
}

impl DivAssign for Volume {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
    }
}

impl CheckedDiv for Volume {
    fn checked_div(&self, v: &Self) -> Option<Self> {
        self.0.checked_div(&v.0).map(|val| Volume(val))
    }
}
