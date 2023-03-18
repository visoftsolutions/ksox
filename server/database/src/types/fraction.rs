use std::{
    cmp::Ordering,
    io::{Error, ErrorKind},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
    str::FromStr,
};

use bigdecimal::{BigDecimal, Zero};
use num_bigint::BigInt;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sqlx::{
    postgres::{PgArgumentBuffer, PgValueRef},
    Decode, Encode, Postgres, Type, TypeInfo,
};
use thiserror::Error;

use super::Volume;

#[derive(Debug, Clone, Serialize, Deserialize, Eq)]
pub struct Fraction {
    #[serde(
        serialize_with = "serialize_bigint_as_string",
        deserialize_with = "deserialize_string_as_bigint"
    )]
    pub numerator: BigInt,
    #[serde(
        serialize_with = "serialize_bigint_as_string",
        deserialize_with = "deserialize_string_as_bigint"
    )]
    pub denominator: BigInt,
}

fn serialize_bigint_as_string<S>(value: &BigInt, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&value.to_string())
}

fn deserialize_string_as_bigint<'de, D>(deserializer: D) -> Result<BigInt, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    BigInt::from_str(&s).map_err(serde::de::Error::custom)
}

impl Fraction {
    fn new(numerator: BigInt, denominator: BigInt) -> Self {
        Fraction {
            numerator,
            denominator,
        }
    }
}

impl FromStr for Fraction {
    type Err = FractionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();

        if parts.len() != 2 {
            return Err(FractionError::Io(Error::new(
                ErrorKind::InvalidData,
                "number of parts not equal to 2",
            )));
        }

        let numerator = BigInt::from_str(parts[0])
            .map_err(|err| Error::new(ErrorKind::InvalidData, err.to_string()))?;
        let denominator = BigInt::from_str(parts[1])
            .map_err(|err| Error::new(ErrorKind::InvalidData, err.to_string()))?;

        Fraction::try_from((numerator, denominator))
    }
}

impl std::fmt::Display for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.numerator, self.denominator)
    }
}

impl Type<Postgres> for Fraction {
    fn type_info() -> <Postgres as sqlx::Database>::TypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("fraction")
    }
    fn compatible(ty: &<Postgres as sqlx::Database>::TypeInfo) -> bool {
        ty.name() == "fraction"
    }
}

impl Decode<'_, Postgres> for Fraction {
    fn decode(value: PgValueRef) -> std::result::Result<Self, sqlx::error::BoxDynError> {
        let value = <(BigDecimal, BigDecimal) as Decode<Postgres>>::decode(value)?;
        TryInto::<Fraction>::try_into((value.0, value.1))
            .map_err(|err| sqlx::error::BoxDynError::from(err.to_string()))
    }
}

impl Encode<'_, Postgres> for Fraction {
    fn produces(&self) -> Option<<Postgres as sqlx::Database>::TypeInfo> {
        Some(sqlx::postgres::PgTypeInfo::with_name("fraction"))
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

impl PartialEq for Fraction {
    fn eq(&self, other: &Self) -> bool {
        self.numerator.to_owned() * other.denominator.to_owned()
            == self.denominator.to_owned() * other.numerator.to_owned()
    }
}

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let lhs = self.numerator.to_owned() * other.denominator.to_owned();
        let rhs = self.denominator.to_owned() * other.numerator.to_owned();
        lhs.partial_cmp(&rhs)
    }
}

impl Ord for Fraction {
    fn cmp(&self, other: &Self) -> Ordering {
        let lhs = self.numerator.to_owned() * other.denominator.to_owned();
        let rhs = self.denominator.to_owned() * other.numerator.to_owned();
        lhs.cmp(&rhs)
    }
    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
    {
        if self < min {
            min
        } else if self > max {
            max
        } else {
            self
        }
    }
    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        if self >= other {
            self
        } else {
            other
        }
    }
    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        if self <= other {
            self
        } else {
            other
        }
    }
}

impl Add<BigInt> for Fraction {
    type Output = Self;
    fn add(self, rhs: BigInt) -> Self::Output {
        Fraction {
            numerator: self.numerator + rhs * self.denominator.to_owned(),
            denominator: self.denominator,
        }
    }
}

impl AddAssign<BigInt> for Fraction {
    fn add_assign(&mut self, rhs: BigInt) {
        self.numerator += rhs * self.denominator.to_owned();
    }
}

impl Sub<BigInt> for Fraction {
    type Output = Self;
    fn sub(self, rhs: BigInt) -> Self::Output {
        Fraction {
            numerator: self.numerator - rhs * self.denominator.to_owned(),
            denominator: self.denominator,
        }
    }
}

impl SubAssign<BigInt> for Fraction {
    fn sub_assign(&mut self, rhs: BigInt) {
        self.numerator -= rhs * self.denominator.to_owned();
    }
}

impl Mul for Fraction {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Fraction {
            numerator: self.numerator * rhs.numerator,
            denominator: self.denominator * rhs.denominator,
        }
    }
}

impl MulAssign for Fraction {
    fn mul_assign(&mut self, rhs: Self) {
        self.numerator *= rhs.numerator;
        self.denominator *= rhs.denominator;
    }
}

impl Div for Fraction {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Fraction {
            numerator: self.numerator * rhs.denominator,
            denominator: self.denominator * rhs.numerator,
        }
    }
}

impl DivAssign for Fraction {
    fn div_assign(&mut self, rhs: Self) {
        self.numerator *= rhs.denominator;
        self.denominator *= rhs.numerator;
    }
}

impl TryFrom<(BigInt, BigInt)> for Fraction {
    type Error = FractionError;
    fn try_from(value: (BigInt, BigInt)) -> Result<Self, Self::Error> {
        if value.1.is_zero() {
            return Err(FractionError::DenominatorIsZero);
        }
        Ok(Fraction::new(value.0, value.1))
    }
}

impl TryFrom<(Volume, Volume)> for Fraction {
    type Error = FractionError;
    fn try_from(value: (Volume, Volume)) -> Result<Self, Self::Error> {
        TryFrom::<(BigInt, BigInt)>::try_from((value.0.into(), value.1.into()))
    }
}

impl TryFrom<(BigDecimal, BigDecimal)> for Fraction {
    type Error = FractionError;
    fn try_from(value: (BigDecimal, BigDecimal)) -> Result<Self, Self::Error> {
        let (num_bigint, num_exp) = value.0.into_bigint_and_exponent();
        let (denum_bigint, denum_exp) = value.1.into_bigint_and_exponent();
        let num = num_bigint
            * BigInt::from(10)
                .pow(TryInto::try_into(-num_exp).map_err(|_| FractionError::NotBigIntConvertable)?);
        let denum = denum_bigint
            * BigInt::from(10).pow(
                TryInto::try_into(-denum_exp).map_err(|_| FractionError::NotBigIntConvertable)?,
            );
        TryFrom::try_from((num, denum))
    }
}

impl Sub<Fraction> for BigInt {
    type Output = Fraction;
    fn sub(self, rhs: Fraction) -> Self::Output {
        Fraction::new(
            self * rhs.denominator.to_owned() - rhs.numerator,
            rhs.denominator,
        )
    }
}

impl Mul<Fraction> for Volume {
    type Output = Volume;
    fn mul(self, rhs: Fraction) -> Self::Output {
        self * rhs.numerator / rhs.denominator
    }
}

#[derive(Error, Debug)]
pub enum FractionError {
    #[error("Denominator can not be zero")]
    DenominatorIsZero,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Could not convert to BigInt")]
    NotBigIntConvertable,
}
