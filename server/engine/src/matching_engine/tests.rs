use database::{
    projections::spot::order::{Order, Status},
    sqlx::{
        types::{chrono::Utc, Uuid},
        Error,
    },
    types::{Fraction, Volume},
};
use futures::{executor::block_on, stream};
use num_bigint::BigInt;
use proptest::{
    prelude::*,
    strategy::Strategy,
    test_runner::{Config, TestRunner},
};

use super::MatchingEngine;

#[tokio::test]
async fn test_two_matching_orders() {
    let request_user_id = Uuid::new_v4();
    let request_quote_asset_id = Uuid::new_v4();
    let request_base_asset_id = Uuid::new_v4();
    let request_quote_asset_volume = Volume::from(BigInt::from(100));
    let request_base_asset_volume = Volume::from(BigInt::from(100));

    let orders: Vec<Result<Order, Error>> = vec![
        Ok(Order {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            user_id: Uuid::new_v4(),
            status: Status::Active,
            quote_asset_id: request_base_asset_id,
            base_asset_id: request_quote_asset_id,
            quote_asset_volume: Volume::from(BigInt::from(100)),
            base_asset_volume: Volume::from(BigInt::from(10)),
        }),
        Ok(Order {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            user_id: Uuid::new_v4(),
            status: Status::Active,
            quote_asset_id: request_base_asset_id,
            base_asset_id: request_quote_asset_id,
            quote_asset_volume: Volume::from(BigInt::from(100)),
            base_asset_volume: Volume::from(BigInt::from(100)),
        }),
    ];

    let result = MatchingEngine::matching_loop(
        request_user_id,
        request_quote_asset_id,
        request_base_asset_id,
        request_quote_asset_volume,
        request_base_asset_volume,
        Box::pin(stream::iter(orders)),
        (BigInt::from(2), BigInt::from(10)).try_into().unwrap(),
        (BigInt::from(1), BigInt::from(10)).try_into().unwrap(),
    )
    .await
    .unwrap();

    assert!(result.orders.is_empty());
    assert!(result.trades.len() == 2);

    assert_eq!(result.trades[0].taker_quote_volume, BigInt::from(10).into());
    assert_eq!(
        result.trades[0].maker_quote_volume,
        BigInt::from(100).into()
    );
    assert_eq!(result.trades[0].taker_base_volume, BigInt::from(80).into());
    assert_eq!(result.trades[0].maker_base_volume, BigInt::from(9).into());

    assert_eq!(result.trades[1].taker_quote_volume, BigInt::from(90).into());
    assert_eq!(result.trades[1].maker_quote_volume, BigInt::from(90).into());
    assert_eq!(result.trades[1].taker_base_volume, BigInt::from(72).into());
    assert_eq!(result.trades[1].maker_base_volume, BigInt::from(81).into());
}

fn arb_maker_order(
    quote_asset_id: Uuid,
    base_asset_id: Uuid,
    quote_asset_balance: u128,
    base_asset_volume: u128,
    quote_asset_volume: u128,
) -> BoxedStrategy<Order> {
    (0..base_asset_volume)
        .prop_flat_map(move |arb_base_asset_volume| {
            Just(
                BigInt::from(quote_asset_balance) * BigInt::from(arb_base_asset_volume)
                    / BigInt::from(quote_asset_volume),
            )
        })
        .prop_flat_map(move |arb_base_volume| {
            Just(Order {
                id: Uuid::new_v4(),
                created_at: Utc::now(),
                user_id: Uuid::new_v4(),
                status: Status::Active,
                quote_asset_id,
                base_asset_id,
                quote_asset_volume: BigInt::from(quote_asset_balance).into(),
                base_asset_volume: arb_base_volume.into(),
            })
        })
        .boxed()
}

const CASES: u32 = 100000;

#[test]
fn proctest_one_matching_order() {
    let request_user_id = Uuid::new_v4();
    let request_quote_asset_id = Uuid::new_v4();
    let request_base_asset_id = Uuid::new_v4();

    TestRunner::new(Config {
        cases: CASES,
        ..Config::default()
    })
    .run(
        &(1..u128::MAX, 1..u128::MAX, 1..u128::MAX).prop_flat_map(
            |(taker_quote_asset_volume, taker_base_asset_volume, maker_quote_asset_balance)| {
                (
                    Just(Volume::from(BigInt::from(taker_quote_asset_volume))),
                    Just(Volume::from(BigInt::from(taker_base_asset_volume))),
                    arb_maker_order(
                        request_base_asset_id,
                        request_quote_asset_id,
                        maker_quote_asset_balance,
                        taker_quote_asset_volume,
                        taker_base_asset_volume,
                    ),
                )
            },
        ),
        |(taker_quote_asset_volume, taker_base_asset_volume, maker_order)| {
            block_on(async {
                let result = MatchingEngine::matching_loop(
                    request_user_id,
                    request_quote_asset_id,
                    request_base_asset_id,
                    taker_quote_asset_volume.to_owned(),
                    taker_base_asset_volume.to_owned(),
                    Box::pin(stream::iter(vec![Ok(maker_order.clone())])),
                    (BigInt::from(0), BigInt::from(1000)).try_into().unwrap(),
                    (BigInt::from(0), BigInt::from(1000)).try_into().unwrap(),
                )
                .await
                .unwrap();

                // assert order created if necessary
                if taker_quote_asset_volume.to_owned() > maker_order.base_asset_volume {
                    assert!(result.orders.len() == 1);
                    assert!(result.trades.len() == 1);
                } else {
                    assert!(result.trades.len() == 1);
                }

                // assert no leaking in quote value
                let mut quote_sum = Volume::from(BigInt::from(0));
                result
                    .orders
                    .iter()
                    .for_each(|order| quote_sum += order.quote_asset_volume.to_owned());
                result
                    .trades
                    .iter()
                    .for_each(|trade| quote_sum += trade.maker_base_volume.to_owned());

                assert_eq!(quote_sum, taker_quote_asset_volume.to_owned());

                // assert no leaking in base value
                if !result.trades.is_empty() {
                    if !result.orders.is_empty() {
                        assert_eq!(
                            (taker_quote_asset_volume.to_owned()
                                - result.trades.first().unwrap().taker_quote_volume.to_owned())
                                * taker_base_asset_volume
                                / taker_quote_asset_volume.to_owned(),
                            result.orders.first().unwrap().base_asset_volume
                        );
                    } else {
                        assert_eq!(
                            (taker_quote_asset_volume.to_owned()
                                - result.trades.first().unwrap().taker_quote_volume.to_owned())
                                * taker_base_asset_volume
                                / taker_quote_asset_volume.to_owned(),
                            BigInt::from(0).into()
                        );
                    }
                }
            });

            Ok(())
        },
    )
    .unwrap();
}
