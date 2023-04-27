use fraction::{num_traits::Inv, Fraction};
use proptest::{prelude::*, prop_compose};
use uuid::Uuid;

use super::arb_fraction::{
    arb_bigger_fraction, arb_fraction_not_bigger_than_one, arb_not_bigger_fraction,
    arb_not_smaller_fraction, arb_smaller_fraction_or_zero,
};
use crate::database::projections::order::OrderGet;

prop_compose! {
    pub fn arb_order()(
        price in any_with::<Fraction>(std::env::var("TESTS_FRACTION_BYTES").unwrap().parse().unwrap()),
        volume in any_with::<Fraction>(std::env::var("TESTS_FRACTION_BYTES").unwrap().parse().unwrap()),
        maker_fee in any_with::<Fraction>(std::env::var("TESTS_FRACTION_BYTES").unwrap().parse().unwrap())
    ) -> OrderGet {
        OrderGet {id: Uuid::new_v4(), price, quote_asset_volume_left: volume, maker_fee}
    }
}

prop_compose! {
    pub fn arb_not_bigger_matching_order(i_p: Fraction, i_v: Fraction)(
        (price, quote_asset_volume_left, maker_fee) in (
            arb_not_smaller_fraction(i_p.inv()),
            arb_fraction_not_bigger_than_one()
        ).prop_flat_map(move |(price, maker_fee)| (
            Just(price.to_owned()),
            arb_not_bigger_fraction(price * i_v.to_owned()),
            Just(maker_fee),
        ))
    ) -> OrderGet {
        OrderGet {id: Uuid::new_v4(), price, quote_asset_volume_left, maker_fee}
    }
}

prop_compose! {
    pub fn arb_not_smaller_matching_order(i_p: Fraction, i_v: Fraction)(
        (price, quote_asset_volume_left, maker_fee) in (
            arb_not_smaller_fraction(i_p.inv()),
            arb_fraction_not_bigger_than_one()
        ).prop_flat_map(move |(price, maker_fee)| (
            Just(price.to_owned()),
            arb_not_smaller_fraction(price * i_v.to_owned()),
            Just(maker_fee),
        ))
    ) -> OrderGet {
        OrderGet {id: Uuid::new_v4(), price, quote_asset_volume_left, maker_fee}
    }
}

prop_compose! {
    pub fn arb_smaller_matching_order(i_p: Fraction, i_v: Fraction)(
        (price, quote_asset_volume_left, maker_fee) in (
            arb_not_smaller_fraction(i_p.inv()),
            arb_fraction_not_bigger_than_one()
        ).prop_flat_map(move |(price, maker_fee)| (
            Just(price.to_owned()),
            arb_smaller_fraction_or_zero(price * i_v.to_owned()),
            Just(maker_fee),
        ))
    ) -> OrderGet {
        OrderGet {id: Uuid::new_v4(), price, quote_asset_volume_left, maker_fee}
    }
}

prop_compose! {
    pub fn arb_bigger_matching_order(i_p: Fraction, i_v: Fraction)(
        (price, quote_asset_volume_left, maker_fee) in (
            arb_not_smaller_fraction(i_p.inv()),
            arb_fraction_not_bigger_than_one()
        ).prop_flat_map(move |(price, maker_fee)| (
            Just(price.to_owned()),
            arb_bigger_fraction(price * i_v.to_owned()),
            Just(maker_fee),
        ))
    ) -> OrderGet {
        OrderGet {id: Uuid::new_v4(), price, quote_asset_volume_left, maker_fee}
    }
}

prop_compose! {
    pub fn arb_not_matching_order(i_p: Fraction)(
        price in arb_smaller_fraction_or_zero(i_p.inv()),
        volume in any_with::<Fraction>(std::env::var("TESTS_FRACTION_BYTES").unwrap().parse().unwrap()),
        maker_fee in arb_fraction_not_bigger_than_one()
    ) -> OrderGet {
        OrderGet {id: Uuid::new_v4(), price, quote_asset_volume_left: volume, maker_fee}
    }
}
