use std::{fmt, ops::Neg, str::FromStr};

use num_bigint::BigInt;
use num_derive::{Num, NumOps, One, Zero};
use num_rational::BigRational;

#[derive(Debug, Clone, PartialEq, PartialOrd, NumOps, One, Zero, Num)]
pub struct Fraction(pub BigRational);

impl Fraction {
    pub fn floor_with_accuracy(self, accuracy: Fraction) -> Fraction {
        Fraction((self.0 / accuracy.0.clone()).floor() * accuracy.0)
    }
    pub fn ceil_with_accuracy(self, accuracy: Fraction) -> Fraction {
        Fraction((self.0 / accuracy.0.clone()).ceil() * accuracy.0)
    }
}

impl Neg for Fraction {
    type Output = Fraction;
    fn neg(self) -> Self {
        Fraction(self.0.neg())
    }
}

// serialization

use serde::{
    de::{self, Deserialize, Deserializer, MapAccess, Visitor},
    ser::{Serialize, SerializeStruct, Serializer},
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

#[cfg(test)]
mod tests {
    use num_bigint::{BigInt, Sign};
    use num_rational::BigRational;
    use num_traits::Zero;
    use proptest::{prelude::*, proptest};

    use super::Fraction;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(10000))]
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
