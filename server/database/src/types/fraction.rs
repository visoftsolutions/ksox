use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use bigdecimal::Zero;
use num_bigint::BigInt;
use thiserror::Error;

use super::Volume;

#[derive(Debug, Clone)]
pub struct Fraction {
    numerator: BigInt,
    denominator: BigInt,
}

#[derive(Error, Debug)]
pub enum FractionError {
    #[error("denominator can not be zero")]
    DenominatorIsZero,
}

impl Fraction {
    fn new(numerator: BigInt, denominator: BigInt) -> Self {
        Fraction {
            numerator,
            denominator,
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
