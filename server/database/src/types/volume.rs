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

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct Volume(BigInt);

impl FromStr for Volume {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(BigInt::from_str(s)?))
    }
}

impl std::fmt::Display for Volume {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<BigInt> for Volume {
    fn from(value: BigInt) -> Self {
        Volume(value)
    }
}

impl From<Volume> for BigInt {
    fn from(val: Volume) -> Self {
        val.0
    }
}

impl From<Volume> for BigDecimal {
    fn from(val: Volume) -> Self {
        BigDecimal::from(val.0)
    }
}

impl Deref for Volume {
    type Target = BigInt;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Serialize for Volume {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        BigDecimal::serialize(&BigDecimal::from(self.0.clone()), serializer)
    }
}

impl<'de> Deserialize<'de> for Volume {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self::from(
            BigDecimal::deserialize(deserializer)?.to_bigint().unwrap(), // save unwraps method always return Some dont know why though
        ))
    }
}

impl Decode<'_, Postgres> for Volume {
    fn decode(value: PgValueRef) -> std::result::Result<Self, sqlx::error::BoxDynError> {
        Ok(Volume(
            <BigDecimal as Decode<Postgres>>::decode(value)?
                .to_bigint()
                .unwrap(), // save unwraps method always return Some dont know why though
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

impl Mul<BigInt> for Volume {
    type Output = Self;
    fn mul(self, rhs: BigInt) -> Self::Output {
        Volume(self.0 * rhs)
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

impl Div<BigInt> for Volume {
    type Output = Self;
    fn div(self, rhs: BigInt) -> Self::Output {
        Volume(self.0 / rhs)
    }
}

impl DivAssign for Volume {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
    }
}

impl CheckedDiv for Volume {
    fn checked_div(&self, v: &Self) -> Option<Self> {
        self.0.checked_div(&v.0).map(Volume)
    }
}

macro_rules! impl_volume_from_int {
    ($T:ty) => {
        impl From<$T> for Volume {
            #[inline]
            fn from(n: $T) -> Self {
                Volume::from(BigInt::from(n as i64))
            }
        }
    };
}

impl_volume_from_int!(i8);
impl_volume_from_int!(i16);
impl_volume_from_int!(i32);
impl_volume_from_int!(isize);

macro_rules! impl_volume_from_uint {
    ($T:ty) => {
        impl From<$T> for Volume {
            #[inline]
            fn from(n: $T) -> Self {
                Volume::from(BigInt::from(n as u64))
            }
        }
    };
}

impl_volume_from_uint!(u8);
impl_volume_from_uint!(u16);
impl_volume_from_uint!(u32);
impl_volume_from_uint!(usize);
