use std::{
    cmp::Ordering,
    ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign},
};

use fraction::Fraction;
use num_bigint::BigInt;
use num_derive::ToPrimitive;
pub use num_traits;
use num_traits::{Inv, Zero};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, ToPrimitive, Serialize, Deserialize)]
pub enum Infinity {
    Positive,
    Negative,
    Zero,
}

impl Neg for Infinity {
    type Output = Self;
    fn neg(self) -> Self {
        match self {
            Infinity::Positive => Infinity::Negative,
            Infinity::Negative => Infinity::Positive,
            Infinity::Zero => Infinity::Zero,
        }
    }
}

impl Inv for Infinity {
    type Output = Fraction;
    fn inv(self) -> Self::Output {
        Fraction::from(BigInt::from(0))
    }
}

impl AddAssign<Fraction> for Infinity {
    fn add_assign(&mut self, _rhs: Fraction) {}
}

impl SubAssign<Fraction> for Infinity {
    fn sub_assign(&mut self, _rhs: Fraction) {}
}

impl Add<Fraction> for Infinity {
    type Output = Infinity;
    fn add(self, _rhs: Fraction) -> Self::Output {
        self
    }
}

impl Sub<Fraction> for Infinity {
    type Output = Infinity;
    fn sub(self, _rhs: Fraction) -> Self::Output {
        self
    }
}

impl Mul<Fraction> for Infinity {
    type Output = Infinity;
    fn mul(self, rhs: Fraction) -> Self::Output {
        match rhs.cmp(&Fraction::zero()) {
            Ordering::Equal => Infinity::Zero,
            Ordering::Greater => self,
            Ordering::Less => self.neg(),
        }
    }
}

impl Div<Fraction> for Infinity {
    type Output = Infinity;
    fn div(self, rhs: Fraction) -> Self::Output {
        match rhs.cmp(&Fraction::zero()) {
            Ordering::Equal => Infinity::Zero,
            Ordering::Greater => self,
            Ordering::Less => self.neg(),
        }
    }
}

impl PartialEq<Fraction> for Infinity {
    fn eq(&self, _other: &Fraction) -> bool {
        false
    }

    fn ne(&self, _other: &Fraction) -> bool {
        true
    }
}

impl PartialOrd<Fraction> for Infinity {
    fn ge(&self, other: &Fraction) -> bool {
        match self {
            Infinity::Positive => true,
            Infinity::Negative => false,
            Infinity::Zero => other.ge(&Fraction::zero()),
        }
    }
    fn gt(&self, other: &Fraction) -> bool {
        match self {
            Infinity::Positive => true,
            Infinity::Negative => false,
            Infinity::Zero => other.gt(&Fraction::zero()),
        }
    }
    fn le(&self, other: &Fraction) -> bool {
        match self {
            Infinity::Positive => false,
            Infinity::Negative => true,
            Infinity::Zero => other.le(&Fraction::zero()),
        }
    }
    fn lt(&self, other: &Fraction) -> bool {
        match self {
            Infinity::Positive => false,
            Infinity::Negative => true,
            Infinity::Zero => other.lt(&Fraction::zero()),
        }
    }
    fn partial_cmp(&self, other: &Fraction) -> Option<Ordering> {
        if other.denom().is_zero() {
            None
        } else {
            match self {
                Infinity::Positive => Some(Ordering::Greater),
                Infinity::Negative => Some(Ordering::Less),
                Infinity::Zero => Fraction::zero().partial_cmp(other),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use proptest::{prelude::*, proptest};
    use seq_macro::seq;

    use crate::Infinity;

    use super::Fraction;

    seq!(N in 0..10 {
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(std::env::var("TESTS_CASES").unwrap().parse().unwrap()))]

        #[test]
        fn positive_infinity_always_greater~N(fraction in any_with::<Fraction>(std::env::var("TESTS_FRACTION_BYTES").unwrap().parse().unwrap())) {
            assert!(Infinity::Positive > fraction);
        }

        #[test]
        fn negative_infinity_always_smaller~N(fraction in any_with::<Fraction>(std::env::var("TESTS_FRACTION_BYTES").unwrap().parse().unwrap())) {
            assert!(Infinity::Negative < fraction);
        }

        #[test]
        fn infinity_remains_on_addition~N(fraction in any_with::<Fraction>(std::env::var("TESTS_FRACTION_BYTES").unwrap().parse().unwrap())) {
            assert_eq!(Infinity::Positive + fraction.to_owned(), Infinity::Positive);
            assert_eq!(Infinity::Negative + fraction.to_owned(), Infinity::Negative);
        }

        #[test]
        fn infinity_remains_on_subtraction~N(fraction in any_with::<Fraction>(std::env::var("TESTS_FRACTION_BYTES").unwrap().parse().unwrap())) {
            assert_eq!(Infinity::Positive - fraction.to_owned(), Infinity::Positive);
            assert_eq!(Infinity::Negative - fraction.to_owned(), Infinity::Negative);
        }
    }
    });
}
