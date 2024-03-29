use fraction::num_traits::Zero;
use proptest::prelude::{any, any_with, Arbitrary};
use proptest::prop_oneof;
use proptest::strategy::{BoxedStrategy, Strategy};
use serde::{Deserialize, Serialize};
use sqlx::database::HasValueRef;
use sqlx::postgres::PgArgumentBuffer;
use sqlx::TypeInfo;
use sqlx::{Database, Decode, Encode, Postgres, Type};
use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Sub};
use thiserror::Error;

use fraction::Fraction;
use infinity::Infinity;
use num_traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub};

#[derive(Debug, Serialize, Deserialize, Clone, Eq)]
pub enum Value {
    Finite(Fraction),
    Infinite(Infinity),
}

impl Value {
    pub fn is_finite(&self) -> bool {
        matches!(*self, Self::Finite(_))
    }

    pub fn is_infinite(&self) -> bool {
        matches!(*self, Self::Infinite(_))
    }

    pub fn into_finite(self) -> Option<Fraction> {
        match self {
            Self::Finite(v) => Some(v),
            Self::Infinite(_) => None,
        }
    }

    pub fn into_infinite(self) -> Option<Infinity> {
        match self {
            Self::Finite(_) => None,
            Self::Infinite(v) => Some(v),
        }
    }
}

impl Add for Value {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Finite(lhs), Value::Finite(rhs)) => Value::Finite(lhs + rhs),
            (Value::Infinite(lhs), Value::Infinite(rhs)) => Value::Infinite(lhs + rhs),
            (Value::Finite(_), Value::Infinite(rhs)) => match rhs {
                Infinity::Positive => Value::Infinite(Infinity::Positive),
                Infinity::Negative => Value::Infinite(Infinity::Negative),
            },
            (Value::Infinite(lhs), Value::Finite(_)) => match lhs {
                Infinity::Positive => Value::Infinite(Infinity::Positive),
                Infinity::Negative => Value::Infinite(Infinity::Negative),
            },
        }
    }
}

impl Sub for Value {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Finite(lhs), Value::Finite(rhs)) => Value::Finite(lhs - rhs),
            (Value::Infinite(lhs), Value::Infinite(rhs)) => Value::Infinite(lhs - rhs),
            (Value::Finite(_), Value::Infinite(rhs)) => match rhs {
                Infinity::Positive => Value::Infinite(Infinity::Negative),
                Infinity::Negative => Value::Infinite(Infinity::Positive),
            },
            (Value::Infinite(lhs), Value::Finite(_)) => match lhs {
                Infinity::Positive => Value::Infinite(Infinity::Positive),
                Infinity::Negative => Value::Infinite(Infinity::Negative),
            },
        }
    }
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Finite(lhs), Value::Finite(rhs)) => Value::Finite(lhs * rhs),
            (Value::Infinite(lhs), Value::Infinite(rhs)) => Value::Infinite(lhs * rhs),
            // handle cases when multiplying finite by infinity
            (Value::Finite(lhs), Value::Infinite(rhs)) => match lhs.cmp(&Fraction::zero()) {
                Ordering::Equal => Value::Finite(Fraction::zero()),
                Ordering::Greater => match rhs {
                    Infinity::Positive => Value::Infinite(Infinity::Positive),
                    Infinity::Negative => Value::Infinite(Infinity::Negative),
                },
                Ordering::Less => match rhs {
                    Infinity::Positive => Value::Infinite(Infinity::Negative),
                    Infinity::Negative => Value::Infinite(Infinity::Positive),
                },
            },
            // handle cases when multiplying infinity by finite
            (Value::Infinite(lhs), Value::Finite(rhs)) => match rhs.cmp(&Fraction::zero()) {
                Ordering::Equal => Value::Finite(Fraction::zero()),
                Ordering::Greater => match lhs {
                    Infinity::Positive => Value::Infinite(Infinity::Positive),
                    Infinity::Negative => Value::Infinite(Infinity::Negative),
                },
                Ordering::Less => match lhs {
                    Infinity::Positive => Value::Infinite(Infinity::Negative),
                    Infinity::Negative => Value::Infinite(Infinity::Positive),
                },
            },
        }
    }
}

impl Div for Value {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Finite(lhs), Value::Finite(rhs)) => Value::Finite(lhs / rhs),
            (Value::Infinite(lhs), Value::Infinite(rhs)) => Value::Infinite(lhs / rhs),
            // handle cases when dividing finite by infinity
            (Value::Finite(_), Value::Infinite(_)) => Value::Finite(Fraction::zero()),
            // handle cases when dividing infinity by finite
            (Value::Infinite(lhs), Value::Finite(rhs)) => match rhs.cmp(&Fraction::zero()) {
                Ordering::Equal => panic!("Invalid division infinity devided by zero"),
                Ordering::Greater => match lhs {
                    Infinity::Positive => Value::Infinite(Infinity::Positive),
                    Infinity::Negative => Value::Infinite(Infinity::Negative),
                },
                Ordering::Less => match lhs {
                    Infinity::Positive => Value::Infinite(Infinity::Negative),
                    Infinity::Negative => Value::Infinite(Infinity::Positive),
                },
            },
        }
    }
}

impl CheckedAdd for Value {
    fn checked_add(&self, v: &Self) -> Option<Self> {
        match (self, v) {
            (Value::Finite(lhs), Value::Finite(rhs)) => lhs.checked_add(rhs).map(Value::Finite),
            (Value::Infinite(lhs), Value::Infinite(rhs)) => {
                lhs.checked_add(rhs).map(Value::Infinite)
            }
            (Value::Finite(_), Value::Infinite(rhs)) => match rhs {
                Infinity::Positive => Some(Value::Infinite(Infinity::Positive)),
                Infinity::Negative => Some(Value::Infinite(Infinity::Negative)),
            },
            (Value::Infinite(lhs), Value::Finite(_)) => match lhs {
                Infinity::Positive => Some(Value::Infinite(Infinity::Positive)),
                Infinity::Negative => Some(Value::Infinite(Infinity::Negative)),
            },
        }
    }
}

impl CheckedSub for Value {
    fn checked_sub(&self, v: &Self) -> Option<Self> {
        match (self, v) {
            (Value::Finite(lhs), Value::Finite(rhs)) => lhs.checked_sub(rhs).map(Value::Finite),
            (Value::Infinite(lhs), Value::Infinite(rhs)) => {
                lhs.checked_sub(rhs).map(Value::Infinite)
            }
            (Value::Finite(_), Value::Infinite(rhs)) => match rhs {
                Infinity::Positive => Some(Value::Infinite(Infinity::Negative)),
                Infinity::Negative => Some(Value::Infinite(Infinity::Positive)),
            },
            (Value::Infinite(lhs), Value::Finite(_)) => match lhs {
                Infinity::Positive => Some(Value::Infinite(Infinity::Positive)),
                Infinity::Negative => Some(Value::Infinite(Infinity::Negative)),
            },
        }
    }
}

impl CheckedMul for Value {
    fn checked_mul(&self, v: &Self) -> Option<Self> {
        match (self, v) {
            (Value::Finite(lhs), Value::Finite(rhs)) => lhs.checked_mul(rhs).map(Value::Finite),
            (Value::Infinite(lhs), Value::Infinite(rhs)) => {
                lhs.checked_mul(rhs).map(Value::Infinite)
            }
            // handle cases when multiplying finite by infinity
            (Value::Finite(lhs), Value::Infinite(rhs)) => match lhs.cmp(&Fraction::zero()) {
                Ordering::Equal => Some(Value::Finite(Fraction::zero())),
                Ordering::Greater => match rhs {
                    Infinity::Positive => Some(Value::Infinite(Infinity::Positive)),
                    Infinity::Negative => Some(Value::Infinite(Infinity::Negative)),
                },
                Ordering::Less => match rhs {
                    Infinity::Positive => Some(Value::Infinite(Infinity::Negative)),
                    Infinity::Negative => Some(Value::Infinite(Infinity::Positive)),
                },
            },
            // handle cases when multiplying infinity by finite
            (Value::Infinite(lhs), Value::Finite(rhs)) => match rhs.cmp(&Fraction::zero()) {
                Ordering::Equal => Some(Value::Finite(Fraction::zero())),
                Ordering::Greater => match lhs {
                    Infinity::Positive => Some(Value::Infinite(Infinity::Positive)),
                    Infinity::Negative => Some(Value::Infinite(Infinity::Negative)),
                },
                Ordering::Less => match lhs {
                    Infinity::Positive => Some(Value::Infinite(Infinity::Negative)),
                    Infinity::Negative => Some(Value::Infinite(Infinity::Positive)),
                },
            },
        }
    }
}

impl CheckedDiv for Value {
    fn checked_div(&self, v: &Self) -> Option<Self> {
        match (self, v) {
            (Value::Finite(lhs), Value::Finite(rhs)) => lhs.checked_div(rhs).map(Value::Finite),
            (Value::Infinite(lhs), Value::Infinite(rhs)) => {
                lhs.checked_div(rhs).map(Value::Infinite)
            }
            // handle cases when dividing finite by infinity
            (Value::Finite(_), Value::Infinite(_)) => Some(Value::Finite(Fraction::zero())),
            // handle cases when dividing infinity by finite
            (Value::Infinite(lhs), Value::Finite(rhs)) => match rhs.cmp(&Fraction::zero()) {
                Ordering::Equal => None,
                Ordering::Greater => match lhs {
                    Infinity::Positive => Some(Value::Infinite(Infinity::Positive)),
                    Infinity::Negative => Some(Value::Infinite(Infinity::Negative)),
                },
                Ordering::Less => match lhs {
                    Infinity::Positive => Some(Value::Infinite(Infinity::Negative)),
                    Infinity::Negative => Some(Value::Infinite(Infinity::Positive)),
                },
            },
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Finite(lhs), Value::Finite(rhs)) => lhs == rhs,
            (Value::Infinite(lhs), Value::Infinite(rhs)) => lhs == rhs,
            _ => false,
        }
    }
}

impl PartialOrd for Value {
    fn ge(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Finite(lhs), Value::Finite(rhs)) => lhs.ge(rhs),
            (Value::Infinite(lhs), Value::Infinite(rhs)) => lhs.ge(rhs),
            (Value::Finite(_), Value::Infinite(rhs)) => match rhs {
                Infinity::Positive => false,
                Infinity::Negative => true,
            },
            (Value::Infinite(lhs), Value::Finite(_)) => match lhs {
                Infinity::Positive => true,
                Infinity::Negative => false,
            },
        }
    }
    fn gt(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Finite(lhs), Value::Finite(rhs)) => lhs.gt(rhs),
            (Value::Infinite(lhs), Value::Infinite(rhs)) => lhs.gt(rhs),
            (Value::Finite(_), Value::Infinite(rhs)) => match rhs {
                Infinity::Positive => false,
                Infinity::Negative => true,
            },
            (Value::Infinite(lhs), Value::Finite(_)) => match lhs {
                Infinity::Positive => true,
                Infinity::Negative => false,
            },
        }
    }
    fn le(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Finite(lhs), Value::Finite(rhs)) => lhs.le(rhs),
            (Value::Infinite(lhs), Value::Infinite(rhs)) => lhs.le(rhs),
            (Value::Finite(_), Value::Infinite(rhs)) => match rhs {
                Infinity::Positive => true,
                Infinity::Negative => false,
            },
            (Value::Infinite(lhs), Value::Finite(_)) => match lhs {
                Infinity::Positive => false,
                Infinity::Negative => true,
            },
        }
    }
    fn lt(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Finite(lhs), Value::Finite(rhs)) => lhs.lt(rhs),
            (Value::Infinite(lhs), Value::Infinite(rhs)) => lhs.lt(rhs),
            (Value::Finite(_), Value::Infinite(rhs)) => match rhs {
                Infinity::Positive => true,
                Infinity::Negative => false,
            },
            (Value::Infinite(lhs), Value::Finite(_)) => match lhs {
                Infinity::Positive => false,
                Infinity::Negative => true,
            },
        }
    }
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Value::Finite(lhs), Value::Finite(rhs)) => lhs.partial_cmp(rhs),
            (Value::Infinite(lhs), Value::Infinite(rhs)) => lhs.partial_cmp(rhs),
            (Value::Finite(_), Value::Infinite(rhs)) => match rhs {
                Infinity::Positive => Some(Ordering::Less),
                Infinity::Negative => Some(Ordering::Greater),
            },
            (Value::Infinite(lhs), Value::Finite(_)) => match lhs {
                Infinity::Positive => Some(Ordering::Greater),
                Infinity::Negative => Some(Ordering::Less),
            },
        }
    }
}

impl Ord for Value {
    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
    {
        match (self.partial_cmp(&min), self.partial_cmp(&max)) {
            (Some(Ordering::Less), _) => min,
            (_, Some(Ordering::Greater)) => max,
            _ => self,
        }
    }
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Value::Finite(lhs), Value::Finite(rhs)) => lhs.cmp(rhs),
            (Value::Infinite(lhs), Value::Infinite(rhs)) => lhs.cmp(rhs),
            (Value::Finite(_), Value::Infinite(rhs)) => match rhs {
                Infinity::Positive => Ordering::Less,
                Infinity::Negative => Ordering::Greater,
            },
            (Value::Infinite(lhs), Value::Finite(_)) => match lhs {
                Infinity::Positive => Ordering::Greater,
                Infinity::Negative => Ordering::Less,
            },
        }
    }
    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        match self.partial_cmp(&other) {
            Some(Ordering::Less) | Some(Ordering::Equal) => other,
            _ => self,
        }
    }
    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        match self.partial_cmp(&other) {
            Some(Ordering::Greater) | Some(Ordering::Equal) => other,
            _ => self,
        }
    }
}

impl Arbitrary for Value {
    type Parameters = usize;
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary() -> Self::Strategy {
        prop_oneof![
            1 => any::<Infinity>().prop_map(Value::Infinite),
            9 => any::<Fraction>().prop_map(Value::Finite),
        ]
        .boxed()
    }

    fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
        prop_oneof![
            1 => any_with::<Infinity>(args).prop_map(Value::Infinite),
            9 => any_with::<Fraction>(args).prop_map(Value::Finite),
        ]
        .boxed()
    }
}

impl TryInto<Fraction> for Value {
    type Error = ValueError;
    fn try_into(self) -> Result<Fraction, Self::Error> {
        match self {
            Self::Finite(f) => Ok(f),
            Self::Infinite(_) => Err(ValueError::ValueInfinite),
        }
    }
}

impl TryInto<Infinity> for Value {
    type Error = ValueError;
    fn try_into(self) -> Result<Infinity, Self::Error> {
        match self {
            Self::Finite(_) => Err(ValueError::ValueFinite),
            Self::Infinite(f) => Ok(f),
        }
    }
}

impl<'r, DB: Database> Decode<'r, DB> for Value
where
    &'r str: Decode<'r, DB>,
{
    fn decode(
        value: <DB as HasValueRef<'r>>::ValueRef,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let value = <&str as Decode<DB>>::decode(value)?;
        Ok(serde_json::from_str(value)?)
    }
}

impl Encode<'_, Postgres> for Value {
    fn produces(&self) -> Option<<Postgres as sqlx::Database>::TypeInfo> {
        Some(sqlx::postgres::PgTypeInfo::with_name("text"))
    }

    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> sqlx::encode::IsNull {
        <String as Encode<Postgres>>::encode_by_ref(
            &serde_json::to_string(self).unwrap_or_default(),
            buf,
        )
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

impl Type<Postgres> for Value {
    fn type_info() -> <Postgres as sqlx::Database>::TypeInfo {
        sqlx::postgres::PgTypeInfo::with_name("value")
    }
    fn compatible(ty: &<Postgres as sqlx::Database>::TypeInfo) -> bool {
        ty.name() == "value"
    }
}

#[derive(Error, Debug)]
pub enum ValueError {
    #[error("value is infinite")]
    ValueInfinite,

    #[error("value is finite")]
    ValueFinite,
}

#[cfg(test)]
mod tests {
    use fraction::Fraction;
    use num_bigint::BigInt;
    use num_traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub};
    use proptest::{prelude::*, proptest};
    use seq_macro::seq;

    use crate::Value;

    seq!(N in 0..10 {
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(std::env::var("TESTS_CASES").unwrap().parse().unwrap()))]

        #[test]
        fn checked_add~N(v1 in any::<Value>(), v2 in any::<Value>()) {
            v1.checked_add(&v2);
        }

        #[test]
        fn checked_sub~N(v1 in any::<Value>(), v2 in any::<Value>()) {
            v1.checked_sub(&v2);
        }

        #[test]
        fn checked_mul~N(v1 in any::<Value>(), v2 in any::<Value>()) {
            v1.checked_mul(&v2);
        }

        #[test]
        fn checked_div~N(v1 in any::<Value>(), v2 in any::<Value>()) {
            v1.checked_div(&v2);
        }
    }
    });

    #[test]
    fn check() {
        let a: Value = serde_json::from_str("{\"Infinite\": \"Positive\"}").unwrap();
        let b = Value::Finite(Fraction::from_raw((BigInt::from(1), BigInt::from(1))).unwrap());

        assert!(a > b);
        assert!(a >= b);

        assert!(a >= b);
        assert!(a > b);
    }
}
