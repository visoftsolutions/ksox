use async_stream::stream;
use futures::executor::block_on;
use num_traits::{Inv, Zero};
use proptest::{collection, prelude::*, proptest};
use uuid::Uuid;

use crate::{
    matching_engine::{
        tests::{
            asset::arb_asset,
            fraction::arb_fraction_bigger_than_zero,
            order::{arb_not_matching_order, arb_smaller_matching_order},
        },
        MatchingEngine,
    },
    types::Fraction,
};

pub mod asset;
pub mod fraction;
pub mod order;

const FRACTION_BYTES: usize = 2;

// ensure that when there is no matching orders available -> new order will be produced with request price and volume for this user
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn new_order_when_no_matching_orders(
        (user_id, price, quote_asset_volume, quote_asset, base_asset, mut not_matching_orders, accuracy) in (
            arb_fraction_bigger_than_zero()
            ).prop_flat_map(|price| (
                Just(Uuid::new_v4()),
                Just(price.to_owned()),
                arb_fraction_bigger_than_zero(),
                arb_asset(),
                arb_asset(),
                collection::vec(arb_not_matching_order(price.inv()), 0..100),
                arb_fraction_bigger_than_zero()
        ))
    ) {
        block_on(async {
            let stream = Box::pin(stream! {
                not_matching_orders.sort_by(|a, b| a.price.cmp(&b.price)) ;
                for order in not_matching_orders {
                    yield Ok(order);
                }
            });

            let matching = MatchingEngine::matching_loop(
                user_id.to_owned(), price.to_owned(), quote_asset_volume.to_owned(), quote_asset.to_owned(), base_asset.to_owned(), stream, accuracy).await.unwrap();

            let order = matching.order.unwrap();
            assert_eq!(order.user_id, user_id);
            assert_eq!(order.quote_asset_id, quote_asset.id);
            assert_eq!(order.base_asset_id, base_asset.id);
            assert_eq!(order.price, price);
            assert_eq!(order.quote_asset_volume, quote_asset_volume);
            assert_eq!(order.quote_asset_volume_left, quote_asset_volume);
            assert_eq!(order.maker_fee, base_asset.maker_fee);
        });
    }
}

// ensure that when there is one smaller matching order available -> there will be trade and leftover order
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    #[test]
    fn new_order_new_trade_when_matching_smaller_order(
        (user_id, price, quote_asset_volume, quote_asset, base_asset, mut not_matching_orders, accuracy) in (
            arb_fraction_bigger_than_zero(),
            arb_fraction_bigger_than_zero()
            ).prop_flat_map(|(price, quote_asset_volume)| (
                Just(Uuid::new_v4()),
                Just(price.to_owned()),
                Just(quote_asset_volume.to_owned()),
                arb_asset(),
                arb_asset(),
                collection::vec(arb_smaller_matching_order(price, quote_asset_volume), 1..=1),
                arb_fraction_bigger_than_zero()
        ))
    ) {
        block_on(async {
            let mut order_volume_zero = false;
            let order_volume_zero_flag = &mut order_volume_zero;

            let stream = Box::pin(stream! {
                not_matching_orders.sort_by(|a, b| a.price.cmp(&b.price)) ;
                for order in not_matching_orders {
                    if order.quote_asset_volume_left <= Fraction::zero() {
                        *order_volume_zero_flag = true;
                    }
                    yield Ok(order);
                }
            });

            let matching = MatchingEngine::matching_loop(
                user_id.to_owned(), price.to_owned(), quote_asset_volume.to_owned(), quote_asset.to_owned(), base_asset.to_owned(), stream, accuracy).await.unwrap();

            assert!(matching.order.is_some());
            assert!(matching.trades.len() > 0 || order_volume_zero);
        });
    }
}
