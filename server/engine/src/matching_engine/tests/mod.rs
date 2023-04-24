use async_stream::stream;
use fraction::{num_traits::Zero, Fraction};
use futures::executor::block_on;
use proptest::{prelude::*, proptest};
use seq_macro::seq;
use uuid::Uuid;

use crate::{
    database::projections::order::OrderGet,
    matching_engine::{
        models::MatchingLoopError,
        tests::{
            arb_asset::arb_asset,
            arb_fraction::arb_fraction_bigger_than_zero,
            arb_order::{
                arb_not_matching_order, arb_not_smaller_matching_order, arb_smaller_matching_order,
            },
        },
        MatchingEngine,
    },
};

pub mod arb_asset;
pub mod arb_fraction;
pub mod arb_order;

seq!(N in 0..10 {
// ensure that when there is not_matching_order supplied -> error is risen
proptest! {
    #![proptest_config(ProptestConfig::with_cases(std::env::var("TESTS_CASES").unwrap().parse().unwrap()))]
    #[test]
    fn error_when_not_matching_order~N(
        (user_id, price, quote_asset_volume, quote_asset, base_asset, maker_order, accuracy) in (
            arb_fraction_bigger_than_zero()
            ).prop_flat_map(|price| (
                Just(Uuid::new_v4()),
                Just(price.to_owned()),
                arb_fraction_bigger_than_zero(),
                arb_asset(),
                arb_asset(),
                arb_not_matching_order(price),
                arb_fraction_bigger_than_zero()
        ))
    ) {
        block_on(async {
            let maker_orders_stream = Box::pin(stream! {
                    yield Ok(maker_order);
            });

            let matching = MatchingEngine::matching_loop(
                user_id.to_owned(), price.to_owned(), quote_asset_volume.to_owned(), quote_asset.to_owned(), base_asset.to_owned(), maker_orders_stream, accuracy).await;

            assert!(matches!(matching, Err(MatchingLoopError::InvalidMatchingOrderData)))
        });
    }
}
});

seq!(N in 0..10 {
// ensure that when there is smaller matching order available -> there will be trade and leftover order
proptest! {
    #![proptest_config(ProptestConfig::with_cases(std::env::var("TESTS_CASES").unwrap().parse().unwrap()))]
    #[test]
    fn new_order_new_trade_when_matching_smaller_order~N(
        (user_id, price, quote_asset_volume, quote_asset, base_asset, maker_order, accuracy) in (
            arb_fraction_bigger_than_zero(),
            arb_fraction_bigger_than_zero()
            ).prop_flat_map(|(price, quote_asset_volume)| (
                Just(Uuid::new_v4()),
                Just(price.to_owned()),
                Just(quote_asset_volume.to_owned()),
                arb_asset(),
                arb_asset(),
                arb_smaller_matching_order(price, quote_asset_volume).prop_filter("order volume not zero", |o| o.quote_asset_volume_left > Fraction::zero()),
                arb_fraction_bigger_than_zero()
        ))
    ) {
        block_on(async {
            let maker_order_copy = maker_order.to_owned();
            let maker_orders_stream = Box::pin(stream! {
                yield Ok(maker_order);
            });

            let matching = MatchingEngine::matching_loop(
                user_id.to_owned(), price.to_owned(), quote_asset_volume.to_owned(), quote_asset.to_owned(), base_asset.to_owned(), maker_orders_stream, accuracy.to_owned()).await.unwrap();

            assert!(matching.order.is_some());
            assert!(!matching.trades.is_empty());
            assert!(matching.trades.len() == 1);

            let trades = matching.trades;
            let trade = &trades[0];
            assert_eq!(trade.taker_id, user_id);
            assert_eq!(trade.order_id, maker_order_copy.id);

            // ensure whole order is used
            let maker_order_base_volume = maker_order_copy.quote_asset_volume_left / maker_order_copy.price;
            assert!(trade.taker_quote_volume == maker_order_base_volume);

            let request_base_asset_volume = maker_order_base_volume.to_owned() / price;
            let minimal_taker_base_volume = (request_base_asset_volume.to_owned() - request_base_asset_volume.to_owned() * base_asset.taker_fee).checked_floor_with_accuracy(&accuracy).unwrap();

            // ensure taker is satisfied
            assert!(trade.taker_base_volume >= minimal_taker_base_volume);

            let minimal_maker_base_volume = (maker_order_base_volume.to_owned() - maker_order_base_volume * maker_order_copy.maker_fee).checked_floor_with_accuracy(&accuracy).unwrap();

            // ensure maker is satisfied
            assert!(trade.maker_base_volume >= minimal_maker_base_volume);


        });
    }
}
});

seq!(N in 0..10 {
// ensure that when there no matching order available -> only there order is created
proptest! {
    #![proptest_config(ProptestConfig::with_cases(std::env::var("TESTS_CASES").unwrap().parse().unwrap()))]
    #[test]
    fn new_order_when_no_matching_order~N(
        (user_id, price, quote_asset_volume, quote_asset, base_asset, maker_order, accuracy) in (
            arb_fraction_bigger_than_zero(),
            arb_fraction_bigger_than_zero()
            ).prop_flat_map(|(price, quote_asset_volume)| (
                Just(Uuid::new_v4()),
                Just(price.to_owned()),
                Just(quote_asset_volume.to_owned()),
                arb_asset(),
                arb_asset(),
                Just(Vec::<OrderGet>::new()),
                arb_fraction_bigger_than_zero()
        ))
    ) {
        block_on(async {
            let maker_orders_stream = Box::pin(stream! {
                for order in maker_order {
                    yield Ok(order);
                }
            });

            let matching = MatchingEngine::matching_loop(
                user_id.to_owned(), price.to_owned(), quote_asset_volume.to_owned(), quote_asset.to_owned(), base_asset.to_owned(), maker_orders_stream, accuracy).await.unwrap();

            assert!(matching.order.is_some());
            assert!(matching.trades.is_empty());

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
});

seq!(N in 0..10 {
// ensure that when there bigger matching order available -> only trade is created
proptest! {
    #![proptest_config(ProptestConfig::with_cases(std::env::var("TESTS_CASES").unwrap().parse().unwrap()))]
    #[test]
    fn new_trade_when_bigger_matching_order~N(
        (user_id, price, quote_asset_volume, quote_asset, base_asset, maker_order, accuracy) in (
            arb_fraction_bigger_than_zero(),
            arb_fraction_bigger_than_zero()
            ).prop_flat_map(|(price, quote_asset_volume)| (
                Just(Uuid::new_v4()),
                Just(price.to_owned()),
                Just(quote_asset_volume.to_owned()),
                arb_asset(),
                arb_asset(),
                arb_not_smaller_matching_order(price, quote_asset_volume).prop_filter("order volume not zero", |o| o.quote_asset_volume_left > Fraction::zero()),
                arb_fraction_bigger_than_zero()
        ))
    ) {
        block_on(async {
            let maker_order_copy = maker_order.to_owned();
            let maker_orders_stream = Box::pin(stream! {
                    yield Ok(maker_order);
            });

            let matching = MatchingEngine::matching_loop(
                user_id.to_owned(), price.to_owned(), quote_asset_volume.to_owned(), quote_asset.to_owned(), base_asset.to_owned(), maker_orders_stream, accuracy.to_owned()).await.unwrap();

            assert!(matching.order.is_none());
            assert!(!matching.trades.is_empty());
            assert!(matching.trades.len() == 1);

            let trades = matching.trades;
            let trade = &trades[0];
            assert_eq!(trade.taker_id, user_id);
            assert_eq!(trade.order_id, maker_order_copy.id);

            // ensure all taker_quote_asset is used
            assert_eq!(trade.taker_quote_volume, quote_asset_volume);

            let request_base_asset_volume = quote_asset_volume.to_owned() / price;
            let minimal_taker_base_volume = (request_base_asset_volume.to_owned() - request_base_asset_volume.to_owned() * base_asset.taker_fee).checked_floor_with_accuracy(&accuracy).unwrap();

            // ensure taker is satisfied
            assert!(trade.taker_base_volume >= minimal_taker_base_volume);

            let matching_order_base_volume = request_base_asset_volume.to_owned() / maker_order_copy.price;
            let minimal_maker_base_volume = (matching_order_base_volume.to_owned() - matching_order_base_volume * maker_order_copy.maker_fee).checked_floor_with_accuracy(&accuracy).unwrap();

            // ensure maker is satisfied
            assert!(trade.maker_base_volume >= minimal_maker_base_volume);
        })
    }
}
});
