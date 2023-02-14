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

const CASES: u32 = 10000000;

#[test]
fn proctest_req_filled_by_matching_order() {
    let request_user_id = Uuid::new_v4();
    let request_quote_asset_id = Uuid::new_v4();
    let request_base_asset_id = Uuid::new_v4();

    TestRunner::new(Config {
        cases: CASES,
        max_local_rejects: CASES * 10,
        ..Config::default()
    })
    .run(
        &(2..u16::MAX, 2..u16::MAX, 1..u16::MAX, 1..u16::MAX)
            .prop_flat_map(
                |(
                    maker_quote_asset_volume,
                    maker_base_asset_volume,
                    taker_fee_denum,
                    maker_fee_denum,
                )| {
                    (
                        (1..maker_base_asset_volume),
                        (1..maker_quote_asset_volume),
                        Just(maker_quote_asset_volume),
                        Just(maker_base_asset_volume),
                        Just(taker_fee_denum),
                        Just(maker_fee_denum),
                    )
                },
            )
            .prop_filter(
                "maker_order price beneficial",
                |(
                    request_quote_asset_volume,
                    request_base_asset_volume,
                    maker_quote_asset_volume,
                    maker_base_asset_volume,
                    taker_fee_denum,
                    maker_fee_denum,
                )| {
                    BigInt::from(*request_quote_asset_volume)
                        * BigInt::from(*maker_quote_asset_volume)
                        >= BigInt::from(*maker_base_asset_volume)
                            * BigInt::from(*request_base_asset_volume)
                },
            )
            .prop_flat_map(
                |(
                    request_quote_asset_volume,
                    request_base_asset_volume,
                    maker_quote_asset_volume,
                    maker_base_asset_volume,
                    taker_fee_denum,
                    maker_fee_denum,
                )| {
                    (
                        Just(request_quote_asset_volume),
                        Just(request_base_asset_volume),
                        Just(maker_quote_asset_volume),
                        Just(maker_base_asset_volume),
                        Just(taker_fee_denum),
                        (0..=taker_fee_denum),
                        Just(maker_fee_denum),
                        (0..=maker_fee_denum),
                    )
                },
            )
            .prop_flat_map(
                |(
                    request_quote_asset_volume,
                    request_base_asset_volume,
                    maker_quote_asset_volume,
                    maker_base_asset_volume,
                    taker_fee_denum,
                    taker_fee_num,
                    maker_fee_denum,
                    maker_fee_num,
                )| {
                    let taker_fee: Fraction =
                        (BigInt::from(taker_fee_num), BigInt::from(taker_fee_denum))
                            .try_into()
                            .unwrap();
                    let maker_fee: Fraction =
                        (BigInt::from(maker_fee_num), BigInt::from(maker_fee_denum))
                            .try_into()
                            .unwrap();
                    (
                        Just(request_quote_asset_volume),
                        Just(request_base_asset_volume),
                        Just(maker_quote_asset_volume),
                        Just(maker_base_asset_volume),
                        Just(taker_fee),
                        Just(maker_fee),
                    )
                },
            )
            .prop_flat_map(
                |(
                    request_quote_asset_volume,
                    request_base_asset_volume,
                    maker_quote_asset_volume,
                    maker_base_asset_volume,
                    taker_fee,
                    maker_fee,
                )| {
                    (
                        Just(Volume::from(BigInt::from(request_quote_asset_volume))),
                        Just(Volume::from(BigInt::from(request_base_asset_volume))),
                        Just(Order {
                            id: Uuid::new_v4(),
                            created_at: Utc::now(),
                            user_id: Uuid::new_v4(),
                            status: Status::Active,
                            quote_asset_id: request_base_asset_id,
                            base_asset_id: request_quote_asset_id,
                            quote_asset_volume: BigInt::from(maker_quote_asset_volume).into(),
                            base_asset_volume: BigInt::from(maker_base_asset_volume).into(),
                        }),
                        Just(taker_fee),
                        Just(maker_fee),
                    )
                },
            ),
        |(
            request_quote_asset_volume,
            request_base_asset_volume,
            maker_order,
            taker_fee,
            maker_fee,
        )| {
            block_on(async {
                assert!(
                    request_quote_asset_volume.to_owned()
                        < maker_order.base_asset_volume.to_owned()
                );
                assert!(
                    request_base_asset_volume.to_owned()
                        < maker_order.quote_asset_volume.to_owned()
                );
                assert!(
                    request_quote_asset_volume.to_owned()
                        * maker_order.quote_asset_volume.to_owned()
                        >= maker_order.base_asset_volume.to_owned()
                            * request_base_asset_volume.to_owned()
                );

                let result = MatchingEngine::matching_loop(
                    request_user_id,
                    request_quote_asset_id,
                    request_base_asset_id,
                    request_quote_asset_volume.to_owned(),
                    request_base_asset_volume.to_owned(),
                    Box::pin(stream::iter(vec![Ok(maker_order.clone())])),
                    taker_fee.to_owned(),
                    maker_fee.to_owned(),
                )
                .await
                .unwrap();

                // assert correct response
                assert!(result.orders.is_empty());
                assert!(result.trades.len() == 1);

                let trade = result.trades.first().unwrap();

                // checks
                assert!(trade.taker_id == request_user_id);
                assert!(trade.order_id == maker_order.id);
                // request totally taken
                assert!(trade.taker_quote_volume == request_quote_asset_volume);
                // got req_base - fee or better
                assert!(
                    trade.taker_base_volume.to_owned()
                        >= request_base_asset_volume.to_owned()
                            - request_base_asset_volume.to_owned() * taker_fee.to_owned()
                );

                // ensure taker fee taken
                assert!(
                    trade.maker_quote_volume.to_owned()
                        - trade.maker_quote_volume.to_owned() * taker_fee.to_owned()
                        == trade.taker_base_volume.to_owned()
                );

                // ensure maker fee taken
                assert!(
                    trade.maker_base_volume
                        == trade.taker_quote_volume.to_owned()
                            - trade.taker_quote_volume.to_owned() * maker_fee
                );
            });

            Ok(())
        },
    )
    .unwrap();
}

#[test]
fn proctest_no_trade_with_not_matching_order() {
    let request_user_id = Uuid::new_v4();
    let request_quote_asset_id = Uuid::new_v4();
    let request_base_asset_id = Uuid::new_v4();

    TestRunner::new(Config {
        cases: CASES,
        max_local_rejects: CASES * 10,
        ..Config::default()
    })
    .run(
        &(
            1..u16::MAX,
            1..u16::MAX,
            1..u16::MAX,
            1..u16::MAX,
            1..u16::MAX,
            1..u16::MAX,
        )
            .prop_filter(
                "maker_order price not beneficial",
                |(
                    request_quote_asset_volume,
                    request_base_asset_volume,
                    maker_quote_asset_volume,
                    maker_base_asset_volume,
                    taker_fee_denum,
                    maker_fee_denum,
                )| {
                    BigInt::from(*request_quote_asset_volume)
                        * BigInt::from(*maker_quote_asset_volume)
                        < BigInt::from(*maker_base_asset_volume)
                            * BigInt::from(*request_base_asset_volume)
                },
            )
            .prop_flat_map(
                |(
                    request_quote_asset_volume,
                    request_base_asset_volume,
                    maker_quote_asset_volume,
                    maker_base_asset_volume,
                    taker_fee_denum,
                    maker_fee_denum,
                )| {
                    (
                        Just(request_quote_asset_volume),
                        Just(request_base_asset_volume),
                        Just(maker_quote_asset_volume),
                        Just(maker_base_asset_volume),
                        Just(taker_fee_denum),
                        (0..=taker_fee_denum),
                        Just(maker_fee_denum),
                        (0..=maker_fee_denum),
                    )
                },
            )
            .prop_flat_map(
                |(
                    request_quote_asset_volume,
                    request_base_asset_volume,
                    maker_quote_asset_volume,
                    maker_base_asset_volume,
                    taker_fee_denum,
                    taker_fee_num,
                    maker_fee_denum,
                    maker_fee_num,
                )| {
                    let taker_fee: Fraction =
                        (BigInt::from(taker_fee_num), BigInt::from(taker_fee_denum))
                            .try_into()
                            .unwrap();
                    let maker_fee: Fraction =
                        (BigInt::from(maker_fee_num), BigInt::from(maker_fee_denum))
                            .try_into()
                            .unwrap();
                    (
                        Just(request_quote_asset_volume),
                        Just(request_base_asset_volume),
                        Just(maker_quote_asset_volume),
                        Just(maker_base_asset_volume),
                        Just(taker_fee),
                        Just(maker_fee),
                    )
                },
            )
            .prop_flat_map(
                |(
                    request_quote_asset_volume,
                    request_base_asset_volume,
                    maker_quote_asset_volume,
                    maker_base_asset_volume,
                    taker_fee,
                    maker_fee,
                )| {
                    (
                        Just(Volume::from(BigInt::from(request_quote_asset_volume))),
                        Just(Volume::from(BigInt::from(request_base_asset_volume))),
                        Just(Order {
                            id: Uuid::new_v4(),
                            created_at: Utc::now(),
                            user_id: Uuid::new_v4(),
                            status: Status::Active,
                            quote_asset_id: request_base_asset_id,
                            base_asset_id: request_quote_asset_id,
                            quote_asset_volume: BigInt::from(maker_quote_asset_volume).into(),
                            base_asset_volume: BigInt::from(maker_base_asset_volume).into(),
                        }),
                        Just(taker_fee),
                        Just(maker_fee),
                    )
                },
            ),
        |(
            request_quote_asset_volume,
            request_base_asset_volume,
            maker_order,
            taker_fee,
            maker_fee,
        )| {
            block_on(async {
                assert!(
                    request_quote_asset_volume.to_owned()
                        * maker_order.quote_asset_volume.to_owned()
                        < maker_order.base_asset_volume.to_owned()
                            * request_base_asset_volume.to_owned()
                );

                let result = MatchingEngine::matching_loop(
                    request_user_id,
                    request_quote_asset_id,
                    request_base_asset_id,
                    request_quote_asset_volume.to_owned(),
                    request_base_asset_volume.to_owned(),
                    Box::pin(stream::iter(vec![Ok(maker_order.clone())])),
                    taker_fee.to_owned(),
                    maker_fee.to_owned(),
                )
                .await
                .unwrap();

                // assert correct response
                assert!(result.trades.is_empty());
                assert!(result.orders.len() == 1);

                let order = result.orders.first().unwrap();

                // checks
                assert!(order.user_id == request_user_id);
                assert!(order.status == Status::Active);
                assert!(order.quote_asset_id == request_quote_asset_id);
                assert!(order.base_asset_id == request_base_asset_id);
                assert!(order.quote_asset_volume == request_quote_asset_volume);
                assert!(order.base_asset_volume == request_base_asset_volume);
            });

            Ok(())
        },
    )
    .unwrap();
}
