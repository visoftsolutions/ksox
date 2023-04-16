use std::{
    fmt,
    ops::{AddAssign, Neg, SubAssign},
    str::FromStr,
};

use num_bigint::{BigInt, Sign};
use num_derive::{Num, NumOps, One, Zero};
use num_rational::BigRational;
use proptest::prelude::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, NumOps, One, Zero, Num, Ord, Eq)]
pub struct Fraction(pub BigRational);

impl Fraction {
    pub fn floor_with_accuracy(self, accuracy: Fraction) -> Fraction {
        Fraction((self.0 / accuracy.0.clone()).floor() * accuracy.0)
    }
    // pub fn ceil_with_accuracy(self, accuracy: Fraction) -> Fraction {
    //     Fraction((self.0 / accuracy.0.clone()).ceil() * accuracy.0)
    // }
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

// serialization

use num_traits::{Inv, Zero};
use proptest::{prelude::Arbitrary, sample::size_range};
use serde::{
    de::{self, Deserialize, Deserializer, MapAccess, Visitor},
    ser::{Serialize, SerializeStruct, Serializer},
};
use sqlx::{
    database::HasValueRef, postgres::PgArgumentBuffer, types::BigDecimal, Database, Decode, Encode,
    Postgres, Type, TypeInfo,
};

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

impl std::fmt::Display for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (numer, denom) = self.0.clone().into();
        write!(f, "({},{})", numer, denom)
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
    use num_bigint::{BigInt, Sign};
    use num_rational::BigRational;
    use num_traits::Zero;
    use proptest::{prelude::*, proptest};

    use super::Fraction;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]
        #[test]
        fn serialization(
            numer in any::<Vec<u8>>().prop_map(|v| BigInt::from_bytes_le(Sign::Plus, v.as_slice())),
            denom in any::<Vec<u8>>().prop_map(|v| BigInt::from_bytes_le(Sign::Plus, v.as_slice())).prop_filter("nonzero", |v| !v.is_zero())
        ) {
            let f = Fraction(BigRational::try_from((numer, denom)).unwrap());
            assert_eq!(f, serde_json::from_str(&serde_json::to_string(&f).unwrap()).unwrap());
        }
    }
}
