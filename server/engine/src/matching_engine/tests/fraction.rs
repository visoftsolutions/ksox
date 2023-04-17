use num_rational::BigRational;
use proptest::{prelude::*, prop_compose};

use super::FRACTION_BYTES;
use crate::types::Fraction;

prop_compose! {
    pub fn arb_fraction_bigger_than_zero()(
        f in any_with::<Fraction>(FRACTION_BYTES)
    ) -> Fraction {
        let (numer, denom) = f.0.into();
        Fraction(BigRational::from((1 + numer, denom)))
    }
}

prop_compose! {
    pub fn arb_fraction_not_bigger_than_one()(
        f in any_with::<Fraction>(FRACTION_BYTES)
    ) -> Fraction {
        let (numer, denom) = f.0.into();
        Fraction(BigRational::from((1 + numer % denom.to_owned(), denom)))
    }
}

prop_compose! {
    pub fn arb_fraction_not_smaller_than_one()(
        f in any_with::<Fraction>(FRACTION_BYTES)
    ) -> Fraction {
        let (numer, denom) = f.0.into();
        Fraction(BigRational::from((numer + denom.to_owned(), denom)))
    }
}
prop_compose! {
    pub fn arb_fraction_smaller_than_one_or_zero()(
        f in any_with::<Fraction>(FRACTION_BYTES)
    ) -> Fraction {
        let (numer, denom) = f.0.into();
        Fraction(BigRational::from((numer % denom.to_owned(), denom)))
    }
}

prop_compose! {
    pub fn arb_fraction_bigger_than_one()(
        f in any_with::<Fraction>(FRACTION_BYTES)
    ) -> Fraction {
        let (numer, denom) = f.0.into();
        Fraction(BigRational::from((1 + numer + denom.to_owned(), denom)))
    }
}

prop_compose! {
    pub fn arb_not_bigger_fraction(f: Fraction)(
        m in arb_fraction_not_bigger_than_one()
    ) -> Fraction {
        m * f.to_owned()
    }
}

prop_compose! {
    pub fn arb_smaller_fraction_or_zero(f: Fraction)(
        m in arb_fraction_smaller_than_one_or_zero()
    ) -> Fraction {
        m * f.to_owned()
    }
}

prop_compose! {
    pub fn arb_bigger_fraction(f: Fraction)(
        m in arb_fraction_bigger_than_one()
    ) -> Fraction {
        m * f.to_owned()
    }
}

prop_compose! {
    pub fn arb_not_smaller_fraction(f: Fraction)(
        m in arb_fraction_not_smaller_than_one()
    ) -> Fraction {
        m * f.to_owned()
    }
}
