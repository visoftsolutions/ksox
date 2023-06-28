use std::{
    fmt::{self, Display},
    ops::{AddAssign, Deref, Neg, SubAssign},
    str::FromStr,
};

use num_bigint::{BigInt, Sign};
use num_derive::{Num, NumOps, One, ToPrimitive, Zero};
use num_rational::{BigRational, ParseRatioError};
pub use num_traits;
use num_traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Inv, One, Signed, Zero};
use proptest::{
    prelude::{Arbitrary, *},
    sample::size_range,
};
use serde::{
    de::{self, Deserialize, Deserializer, MapAccess, Visitor},
    ser::{Serialize, SerializeStruct, Serializer},
};
use sqlx::{
    database::HasValueRef, postgres::PgArgumentBuffer, types::BigDecimal, Database, Decode, Encode,
    Postgres, Type, TypeInfo,
};

#[derive(
    Debug, Clone, PartialEq, PartialOrd, NumOps, One, Zero, Num, Ord, Eq, Default, ToPrimitive, Hash,
)]
pub struct Fraction(pub BigRational);

impl Fraction {
    pub fn checked_floor_with_accuracy(self, accuracy: &Self) -> Option<Self> {
        Some(Self(
            self.0
                .checked_div(&accuracy.0)?
                .floor()
                .checked_mul(&accuracy.0)?,
        ))
    }

    pub fn checked_round_with_accuracy(self, accuracy: &Self) -> Option<Self> {
        Some(Self(
            self.0
                .checked_div(&accuracy.0)?
                .round()
                .checked_mul(&accuracy.0)?,
        ))
    }

    pub fn checked_ceil_with_accuracy(self, accuracy: &Self) -> Option<Self> {
        Some(Self(
            self.0
                .checked_div(&accuracy.0)?
                .ceil()
                .checked_mul(&accuracy.0)?,
        ))
    }

    pub fn to_tuple_string(&self) -> String {
        let (numer, denom) = self.0.clone().into();
        format!("({},{})", numer, denom)
    }

    pub fn from_raw(data: (BigInt, BigInt)) -> Option<Self> {
        if data.1 == BigInt::zero() {
            return None;
        }
        Some(Self(BigRational::new_raw(data.0, data.1)))
    }
    
    pub fn numer(&self) -> &BigInt {
        self.0.numer()
    }

    pub fn denom(&self) -> &BigInt {
        self.0.denom()
    }
}

impl Neg for Fraction {
    type Output = Self;
    fn neg(self) -> Self {
        Fraction(self.0.neg())
    }
}

impl Inv for Fraction {
    type Output = Self;
    fn inv(self) -> Self::Output {
        Fraction(self.0.inv())
    }
}

impl Deref for Fraction {
    type Target = BigRational;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AddAssign for Fraction {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl SubAssign for Fraction {
    fn sub_assign(&mut self, rhs: Fraction) {
        self.0 -= rhs.0;
    }
}

impl CheckedAdd for Fraction {
    fn checked_add(&self, v: &Self) -> Option<Self> {
        Some(Fraction(self.0.checked_add(&v.0)?))
    }
}

impl CheckedSub for Fraction {
    fn checked_sub(&self, v: &Self) -> Option<Self> {
        Some(Fraction(self.0.checked_sub(&v.0)?))
    }
}

impl CheckedMul for Fraction {
    fn checked_mul(&self, v: &Self) -> Option<Self> {
        Some(Fraction(self.0.checked_mul(&v.0)?))
    }
}

impl CheckedDiv for Fraction {
    fn checked_div(&self, v: &Self) -> Option<Self> {
        Some(Fraction(self.0.checked_div(&v.0)?))
    }
}

impl From<(BigInt, BigInt)> for Fraction {
    fn from(value: (BigInt, BigInt)) -> Self {
        Fraction(BigRational::from(value))
    }
}

impl From<usize> for Fraction {
    fn from(value: usize) -> Self {
        Self(BigRational::from_integer(value.into()))
    }
}

impl FromStr for Fraction {
    type Err = ParseRatioError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(BigRational::from_str(s)?))
    }
}

impl Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Signed for Fraction {
    fn abs(&self) -> Self {
        Fraction(self.0.abs())
    }
    fn abs_sub(&self, other: &Self) -> Self {
        Fraction(self.0.abs_sub(&other.0))
    }
    fn signum(&self) -> Self {
        if self.is_positive() {
            Self::one()
        } else if self.is_zero() {
            Self::zero()
        } else {
            -Self::one()
        }
    }
    fn is_negative(&self) -> bool {
        self.0.is_negative()
    }
    fn is_positive(&self) -> bool {
        self.0.is_positive()
    }
}

// serialization

const STRUCT: &str = "Fraction";
const FIELDS: &[&str] = &["numer", "denom"];

impl Serialize for Fraction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let (numer, denom) = (self.0.numer(), self.0.denom());
        let mut state = serializer.serialize_struct(STRUCT, 2)?;
        state.serialize_field(FIELDS[0], numer.to_string().as_str())?;
        state.serialize_field(FIELDS[1], denom.to_string().as_str())?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Fraction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Numer,
            Denom,
        }

        struct FractionVisitor;

        impl<'de> Visitor<'de> for FractionVisitor {
            type Value = Fraction;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str(STRUCT)
            }

            fn visit_map<V>(self, mut map: V) -> Result<Fraction, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut numer: Option<BigInt> = None;
                let mut denom: Option<BigInt> = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Numer => {
                            if numer.is_some() {
                                return Err(de::Error::duplicate_field(FIELDS[0]));
                            }
                            numer = Some(
                                BigInt::from_str(map.next_value::<String>()?.as_str())
                                    .map_err(de::Error::custom)?,
                            );
                        }
                        Field::Denom => {
                            if denom.is_some() {
                                return Err(de::Error::duplicate_field(FIELDS[1]));
                            }
                            denom = Some(
                                BigInt::from_str(map.next_value::<String>()?.as_str())
                                    .map_err(de::Error::custom)?,
                            );
                        }
                    }
                }
                let numer = numer.ok_or_else(|| de::Error::missing_field(FIELDS[0]))?;
                let denom = denom.ok_or_else(|| de::Error::missing_field(FIELDS[1]))?;
                Ok(Fraction(
                    BigRational::try_from((numer, denom)).map_err(de::Error::custom)?,
                ))
            }
        }

        deserializer.deserialize_struct(STRUCT, FIELDS, FractionVisitor)
    }
}

impl<'r, DB: Database> Decode<'r, DB> for Fraction
where
    (BigDecimal, BigDecimal): Decode<'r, DB>,
{
    fn decode(
        value: <DB as HasValueRef<'r>>::ValueRef,
    ) -> Result<Fraction, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let (numer, denom) = <(BigDecimal, BigDecimal) as Decode<DB>>::decode(value)?;

        let (numer_bigint, numer_exp) = numer.into_bigint_and_exponent();
        let (denom_bigint, denom_exp) = denom.into_bigint_and_exponent();
        let numer = numer_bigint * BigInt::from(10).pow(TryInto::try_into(-numer_exp)?);
        let denom = denom_bigint * BigInt::from(10).pow(TryInto::try_into(-denom_exp)?);
        Ok(Fraction(BigRational::try_from((numer, denom))?))
    }
}

impl Encode<'_, Postgres> for Fraction {
    fn produces(&self) -> Option<<Postgres as sqlx::Database>::TypeInfo> {
        Some(sqlx::postgres::PgTypeInfo::with_name("fraction"))
    }

    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> sqlx::encode::IsNull {
        <String as Encode<Postgres>>::encode_by_ref(&self.to_tuple_string(), buf)
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

impl Type<Postgres> for Fraction {
    fn type_info() -> <Postgres as sqlx::Database>::TypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("fraction")
    }
    fn compatible(ty: &<Postgres as sqlx::Database>::TypeInfo) -> bool {
        ty.name() == "fraction"
    }
}

impl Arbitrary for Fraction {
    type Parameters = usize;
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary() -> Self::Strategy {
        let sign = Sign::Plus;
        let numer = any::<Vec<u8>>().prop_map(move |v| BigInt::from_bytes_le(sign, v.as_slice()));
        let denom = any::<Vec<u8>>()
            .prop_map(move |v| BigInt::from_bytes_le(sign, v.as_slice()))
            .prop_map(|n| if n.is_zero() { n + 1 } else { n });
        (numer, denom)
            .prop_map(|(numer, denom)| Fraction(BigRational::from((numer, denom))))
            .boxed()
    }

    fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
        let sign = Sign::Plus;
        let numer = any_with::<Vec<u8>>(size_range(args).lift())
            .prop_map(move |v| BigInt::from_bytes_le(sign, v.as_slice()));
        let denom = any_with::<Vec<u8>>(size_range(args).lift())
            .prop_map(move |v| BigInt::from_bytes_le(sign, v.as_slice()))
            .prop_map(|n| if n.is_zero() { n + 1 } else { n });
        (numer, denom)
            .prop_map(|(numer, denom)| Fraction(BigRational::from((numer, denom))))
            .boxed()
    }
}

#[cfg(test)]
mod tests {
    use proptest::{prelude::*, proptest};
    use seq_macro::seq;

    use super::Fraction;

    seq!(N in 0..10 {
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(std::env::var("TESTS_CASES").unwrap().parse().unwrap()))]
        #[test]
        fn serialization~N(fraction in any_with::<Fraction>(std::env::var("TESTS_FRACTION_BYTES").unwrap().parse().unwrap())) {
            assert_eq!(fraction, serde_json::from_str(&serde_json::to_string(&fraction).unwrap()).unwrap());
        }
    }
    });
}
