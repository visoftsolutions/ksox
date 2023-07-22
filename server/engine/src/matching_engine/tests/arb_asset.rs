use fraction::Fraction;
use num_bigint::BigInt;
use proptest::prop_compose;
use uuid::Uuid;

use super::arb_fraction::arb_fraction_not_bigger_than_one;
use crate::database::projections::asset::Asset;

prop_compose! {
    pub fn arb_asset()(
        decimals in 1..1000000000000000000000u128,
        maker_fee in arb_fraction_not_bigger_than_one(),
        taker_fee in arb_fraction_not_bigger_than_one(),
    ) -> Asset {
        Asset {id: Uuid::new_v4(), decimals: Fraction::from((BigInt::from(decimals), BigInt::from(1))), maker_fee, taker_fee}
    }
}
