use std::{
    cmp::Ordering,
    ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign},
};

use num_derive::ToPrimitive;
pub use num_traits;
use num_traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub};
use proptest::{
    prelude::{any, Arbitrary},
    strategy::{BoxedStrategy, Strategy},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, ToPrimitive, Serialize, Deserialize)]
pub enum Infinity {
    Positive,
    Negative,
}

impl Neg for Infinity {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            Self::Positive => Self::Negative,
            Self::Negative => Self::Positive,
        }
    }
}

impl Add for Infinity {
    type Output = Infinity;
    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            (Self::Positive, Self::Positive) => Self::Positive,
            (Self::Negative, Self::Negative) => Self::Negative,
            _ => panic!("Invalid addition between infinities"),
        }
    }
}

impl Sub for Infinity {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        match (self, other) {
            (Self::Positive, Self::Negative) => Self::Positive,
            (Self::Negative, Self::Positive) => Self::Negative,
            _ => panic!("Invalid subtraction between infinities"),
        }
    }
}

impl Mul for Infinity {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        match (self, other) {
            (Self::Positive, Self::Positive) => Self::Positive,
            (Self::Negative, Self::Negative) => Self::Positive,
            (Self::Positive, Self::Negative) => Self::Negative,
            (Self::Negative, Self::Positive) => Self::Negative,
        }
    }
}

impl Div for Infinity {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        match (self, other) {
            (Self::Positive, Self::Positive) => Self::Positive,
            (Self::Negative, Self::Negative) => Self::Positive,
            (Self::Positive, Self::Negative) => Self::Negative,
            (Self::Negative, Self::Positive) => Self::Negative,
        }
    }
}

impl AddAssign for Infinity {
    fn add_assign(&mut self, other: Self) {
        match (&self, other) {
            (Self::Positive, Self::Positive) => *self = Self::Positive,
            (Self::Negative, Self::Negative) => *self = Self::Negative,
            _ => panic!("Invalid addition assignment between infinities"),
        }
    }
}

impl SubAssign for Infinity {
    fn sub_assign(&mut self, other: Self) {
        match (&self, other) {
            (Self::Positive, Self::Negative) => *self = Self::Positive,
            (Self::Negative, Self::Positive) => *self = Self::Negative,
            _ => panic!("Invalid subtraction assignment between infinities"),
        }
    }
}

impl PartialEq for Infinity {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Positive, Self::Positive) => true,
            (Self::Negative, Self::Negative) => true,
            _ => false,
        }
    }
}

impl PartialOrd for Infinity {
    fn ge(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Positive, Self::Positive) => true,
            (Self::Negative, Self::Negative) => true,
            (Self::Negative, Self::Positive) => false,
            (Self::Positive, Self::Negative) => true,
        }
    }
    fn gt(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Positive, Self::Positive) => false,
            (Self::Negative, Self::Negative) => false,
            (Self::Negative, Self::Positive) => false,
            (Self::Positive, Self::Negative) => true,
        }
    }
    fn le(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Positive, Self::Positive) => true,
            (Self::Negative, Self::Negative) => true,
            (Self::Negative, Self::Positive) => true,
            (Self::Positive, Self::Negative) => false,
        }
    }
    fn lt(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Positive, Self::Positive) => false,
            (Self::Negative, Self::Negative) => false,
            (Self::Negative, Self::Positive) => true,
            (Self::Positive, Self::Negative) => false,
        }
    }
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Positive, Self::Positive) => Some(Ordering::Equal),
            (Self::Negative, Self::Negative) => Some(Ordering::Equal),
            (Self::Negative, Self::Positive) => Some(Ordering::Less),
            (Self::Positive, Self::Negative) => Some(Ordering::Greater),
        }
    }
}

impl CheckedAdd for Infinity {
    fn checked_add(&self, v: &Self) -> Option<Self> {
        match (self, v) {
            (Self::Positive, Self::Positive) => Some(Self::Positive),
            (Self::Negative, Self::Negative) => Some(Self::Negative),
            _ => None,
        }
    }
}

impl CheckedSub for Infinity {
    fn checked_sub(&self, v: &Self) -> Option<Self> {
        match (self, v) {
            (Self::Positive, Self::Negative) => Some(Self::Positive),
            (Self::Negative, Self::Positive) => Some(Self::Negative),
            _ => None,
        }
    }
}

impl CheckedMul for Infinity {
    fn checked_mul(&self, v: &Self) -> Option<Self> {
        match (self, v) {
            (Self::Positive, Self::Positive) => Some(Self::Positive),
            (Self::Negative, Self::Negative) => Some(Self::Positive),
            (Self::Positive, Self::Negative) => Some(Self::Negative),
            (Self::Negative, Self::Positive) => Some(Self::Negative),
        }
    }
}

impl CheckedDiv for Infinity {
    fn checked_div(&self, v: &Self) -> Option<Self> {
        match (self, v) {
            (Self::Positive, Self::Positive) => Some(Self::Positive),
            (Self::Negative, Self::Negative) => Some(Self::Positive),
            (Self::Positive, Self::Negative) => Some(Self::Negative),
            (Self::Negative, Self::Positive) => Some(Self::Negative),
        }
    }
}

impl Arbitrary for Infinity {
    type Parameters = usize;
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary() -> Self::Strategy {
        any::<bool>()
            .prop_map(|f| match f {
                true => Infinity::Positive,
                false => Infinity::Negative,
            })
            .boxed()
    }

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        Self::arbitrary()
    }
}

#[cfg(test)]
mod tests {
    use num_traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub};
    use proptest::{prelude::*, proptest};
    use seq_macro::seq;

    use crate::Infinity;

    seq!(N in 0..10 {
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(std::env::var("TESTS_CASES").unwrap().parse().unwrap()))]

        #[test]
        fn checked_add~N(inf1 in any::<Infinity>(), inf2 in any::<Infinity>()) {
            inf1.checked_add(&inf2);
        }

        #[test]
        fn checked_sub~N(inf1 in any::<Infinity>(), inf2 in any::<Infinity>()) {
            inf1.checked_sub(&inf2);
        }

        #[test]
        fn checked_mul~N(inf1 in any::<Infinity>(), inf2 in any::<Infinity>()) {
            inf1.checked_mul(&inf2);
        }

        #[test]
        fn checked_div~N(inf1 in any::<Infinity>(), inf2 in any::<Infinity>()) {
            inf1.checked_div(&inf2);
        }
    }
    });
}
