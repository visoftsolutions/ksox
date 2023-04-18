use proptest::prop_compose;
use uuid::Uuid;

use crate::database::projections::asset::Asset;

use super::arb_fraction::arb_fraction_not_bigger_than_one;

prop_compose! {
    pub fn arb_asset()(
        maker_fee in arb_fraction_not_bigger_than_one(),
        taker_fee in arb_fraction_not_bigger_than_one(),
    ) -> Asset {
        Asset {id: Uuid::new_v4(), maker_fee, taker_fee}
    }
}
