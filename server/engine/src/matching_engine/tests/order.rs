use num_traits::Inv;
use proptest::{prelude::*, prop_compose};
use uuid::Uuid;

use super::{
    fraction::{
        arb_bigger_fraction, arb_fraction_not_bigger_than_one, arb_not_bigger_fraction,
        arb_not_smaller_fraction, arb_smaller_fraction_or_zero,
    },
    FRACTION_BYTES,
};
use crate::{database::OrderGet, types::Fraction};

prop_compose! {
    pub fn arb_order()(
        price in any_with::<Fraction>(FRACTION_BYTES),
        volume in any_with::<Fraction>(FRACTION_BYTES),
        fee in any_with::<Fraction>(FRACTION_BYTES)
    ) -> OrderGet {
        OrderGet {id: Uuid::new_v4(), price, quote_asset_volume_left: volume, maker_fee: fee}
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
        volume in any_with::<Fraction>(FRACTION_BYTES),
        fee in arb_fraction_not_bigger_than_one()
    ) -> OrderGet {
        OrderGet {id: Uuid::new_v4(), price, quote_asset_volume_left: volume, maker_fee: fee}
    }
}
