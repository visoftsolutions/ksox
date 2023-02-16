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
use seq_macro::seq;
use tokio_stream::StreamExt;

use super::MatchingEngine;

// maximum 1000000000
const CASES: u32 = 1000000;
const MAX_LOCAL_REJECTS: u32 = CASES * 3;

fn arb_fee(denum: u16) -> BoxedStrategy<Fraction> {
    (0..=denum)
        .prop_flat_map(move |num| {
            Just(TryInto::<Fraction>::try_into((BigInt::from(num), BigInt::from(denum))).unwrap())
        })
        .boxed()
}

fn arb_smaller_matching_order(
    request_quote_asset_id: Uuid,
    request_base_asset_id: Uuid,
    request_quote_asset_volume: u16,
    request_base_asset_volume: u16,
) -> BoxedStrategy<Order> {
    (
        Just(request_quote_asset_volume),
        Just(request_base_asset_volume),
        1..=request_base_asset_volume,
        1..=request_quote_asset_volume,
    )
        .prop_filter(
            "maker_order price beneficial",
            |(
                request_quote_asset_volume,
                request_base_asset_volume,
                maker_quote_asset_volume,
                maker_base_asset_volume,
            )| {
                BigInt::from(*request_quote_asset_volume) * BigInt::from(*maker_quote_asset_volume)
                    >= BigInt::from(*maker_base_asset_volume)
                        * BigInt::from(*request_base_asset_volume)
            },
        )
        .prop_flat_map(
            move |(
                request_quote_asset_volume,
                request_base_asset_volume,
                maker_quote_asset_volume,
                maker_base_asset_volume,
            )| {
                Just(Order {
                    id: Uuid::new_v4(),
                    created_at: Utc::now(),
                    user_id: Uuid::new_v4(),
                    status: Status::Active,
                    quote_asset_id: request_base_asset_id,
                    base_asset_id: request_quote_asset_id,
                    quote_asset_volume: Volume::from(BigInt::from(maker_quote_asset_volume)),
                    base_asset_volume: Volume::from(BigInt::from(maker_base_asset_volume)),
                })
            },
        )
        .boxed()
}

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

seq!(N in 0x000..0x00f {
    #[test]
    fn proctest_req_fully_filled_by_matching_order~N() {
        let request_user_id = Uuid::new_v4();
        let request_quote_asset_id = Uuid::new_v4();
        let request_base_asset_id = Uuid::new_v4();

        TestRunner::new(Config {
            cases: CASES,
            max_local_rejects: MAX_LOCAL_REJECTS,
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
                            (1..=maker_base_asset_volume),
                            (1..=maker_quote_asset_volume),
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
                            Just(Volume::from(BigInt::from(request_quote_asset_volume))),
                            Just(Volume::from(BigInt::from(request_base_asset_volume))),
                            Just(Order {
                                id: Uuid::new_v4(),
                                created_at: Utc::now(),
                                user_id: Uuid::new_v4(),
                                status: Status::Active,
                                quote_asset_id: request_base_asset_id,
                                base_asset_id: request_quote_asset_id,
                                quote_asset_volume: Volume::from(BigInt::from(maker_quote_asset_volume)),
                                base_asset_volume: Volume::from(BigInt::from(maker_base_asset_volume)),
                            }),
                            arb_fee(taker_fee_denum),
                            arb_fee(maker_fee_denum),
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
                            <= maker_order.base_asset_volume.to_owned()
                    );
                    assert!(
                        request_base_asset_volume.to_owned()
                            <= maker_order.quote_asset_volume.to_owned()
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
});

seq!(N in 0x000..0x00f {
    #[test]
    fn proctest_no_trade_with_not_matching_order~N() {
        let request_user_id = Uuid::new_v4();
        let request_quote_asset_id = Uuid::new_v4();
        let request_base_asset_id = Uuid::new_v4();

        TestRunner::new(Config {
            cases: CASES,
            max_local_rejects: MAX_LOCAL_REJECTS,
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
                            Just(Volume::from(BigInt::from(request_quote_asset_volume))),
                            Just(Volume::from(BigInt::from(request_base_asset_volume))),
                            Just(Order {
                                id: Uuid::new_v4(),
                                created_at: Utc::now(),
                                user_id: Uuid::new_v4(),
                                status: Status::Active,
                                quote_asset_id: request_base_asset_id,
                                base_asset_id: request_quote_asset_id,
                                quote_asset_volume: Volume::from(BigInt::from(maker_quote_asset_volume)),
                                base_asset_volume: Volume::from(BigInt::from(maker_base_asset_volume)),
                            }),
                            arb_fee(taker_fee_denum),
                            arb_fee(maker_fee_denum),
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
});

seq!(N in 0x000..0x00f {
    #[test]
    fn proctest_req_partially_filled_by_matching_order~N() {
    let request_user_id = Uuid::new_v4();
    let request_quote_asset_id = Uuid::new_v4();
    let request_base_asset_id = Uuid::new_v4();

    TestRunner::new(Config {
        cases: CASES,
        max_local_rejects: MAX_LOCAL_REJECTS,
        ..Config::default()
    })
    .run(
        &(1..u16::MAX-1, 1..u16::MAX-1, 1..u16::MAX, 1..u16::MAX)
            .prop_flat_map(
                |(
                    maker_quote_asset_volume,
                    maker_base_asset_volume,
                    taker_fee_denum,
                    maker_fee_denum,
                )| {
                    (
                        (maker_base_asset_volume+1..=u16::MAX),
                        (maker_quote_asset_volume+1..=u16::MAX),
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
                        Just(Volume::from(BigInt::from(request_quote_asset_volume))),
                        Just(Volume::from(BigInt::from(request_base_asset_volume))),
                        Just(Order {
                            id: Uuid::new_v4(),
                            created_at: Utc::now(),
                            user_id: Uuid::new_v4(),
                            status: Status::Active,
                            quote_asset_id: request_base_asset_id,
                            base_asset_id: request_quote_asset_id,
                            quote_asset_volume: Volume::from(BigInt::from(maker_quote_asset_volume)),
                            base_asset_volume: Volume::from(BigInt::from(maker_base_asset_volume)),
                        }),
                        arb_fee(taker_fee_denum),
                        arb_fee(maker_fee_denum),
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
                        > maker_order.base_asset_volume.to_owned()
                );
                assert!(
                    request_base_asset_volume.to_owned()
                        > maker_order.quote_asset_volume.to_owned()
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
                assert!(result.orders.len() == 1);
                assert!(result.trades.len() == 1);

                let order = result.orders.first().unwrap();
                let trade = result.trades.first().unwrap();

                // checks
                assert!(order.user_id == request_user_id);
                assert!(order.status == Status::Active);
                assert!(order.quote_asset_id == request_quote_asset_id);
                assert!(order.base_asset_id == request_base_asset_id);

                assert!(trade.taker_id == request_user_id);
                assert!(trade.order_id == maker_order.id);

                // maker order totally taken
                assert!(trade.taker_quote_volume == maker_order.base_asset_volume);

                // ensure taker fee taken
                assert!(
                    trade.taker_base_volume.to_owned() == maker_order.quote_asset_volume.to_owned() - maker_order.quote_asset_volume.to_owned() * taker_fee.to_owned()
                );

                // ensure maker fee taken
                assert!(
                    trade.maker_base_volume
                        == trade.taker_quote_volume.to_owned() - trade.taker_quote_volume.to_owned() * maker_fee
                );
            });

            Ok(())
        },
    )
    .unwrap();
}
});

seq!(N in 0x000..0x00f {
    #[test]
    fn proctest_req_filled_by_matching_orders_vec~N() {
        let request_user_id = Uuid::new_v4();
        let request_quote_asset_id = Uuid::new_v4();
        let request_base_asset_id = Uuid::new_v4();

        TestRunner::new(Config {
            cases: CASES,
            max_local_rejects: MAX_LOCAL_REJECTS*50,
            ..Config::default()
        })
        .run(
            &(1..u16::MAX, 1..u16::MAX, 1..u16::MAX, 1..u16::MAX).prop_flat_map(
                |(
                    request_quote_asset_volume,
                    request_base_asset_volume,
                    taker_fee_denum,
                    maker_fee_denum,
                )| {
                    (
                        Just(Volume::from(BigInt::from(request_quote_asset_volume))),
                        Just(Volume::from(BigInt::from(request_base_asset_volume))),
                        prop::collection::vec(
                            arb_smaller_matching_order(
                                request_quote_asset_id,
                                request_base_asset_id,
                                request_quote_asset_volume,
                                request_base_asset_volume,
                            ),
                            1..=100,
                        ),
                        arb_fee(taker_fee_denum),
                        arb_fee(maker_fee_denum),
                    )
                },
            ),
            |(
                request_quote_asset_volume,
                request_base_asset_volume,
                mut matching_orders,
                taker_fee,
                maker_fee,
            )| {
                block_on(async {
                    matching_orders.sort_by(|lhs, rhs| {
                        (lhs.base_asset_volume.to_owned() * rhs.quote_asset_volume.to_owned()).cmp(
                            &(rhs.base_asset_volume.to_owned() * lhs.quote_asset_volume.to_owned()),
                        )
                    });

                    let mut trades_counter = 0;
                    let mut matching_orders_iter = matching_orders.iter();
                    let mut request_quote_asset_volume_left = request_quote_asset_volume.to_owned();
                    while let Some(order) = matching_orders_iter.next() && request_quote_asset_volume_left > Volume::from(BigInt::from(0)) {
                        request_quote_asset_volume_left -= order.base_asset_volume.to_owned();
                        trades_counter += 1;
                    }

                    let result = MatchingEngine::matching_loop(
                        request_user_id,
                        request_quote_asset_id,
                        request_base_asset_id,
                        request_quote_asset_volume,
                        request_base_asset_volume,
                        Box::pin(
                            stream::iter(matching_orders.to_owned())
                                .map::<Result<Order, Error>, _>(|order| Ok(order)),
                        ),
                        taker_fee,
                        maker_fee,
                    )
                    .await
                    .unwrap();

                    assert!(result.trades.len() == trades_counter);
                    for (idx, trade) in result.trades.iter().enumerate() {
                        assert!(trade.taker_id == request_user_id);
                        assert!(trade.order_id == matching_orders[idx].id);
                    }
                });
                Ok(())
            },
        )
        .unwrap();
    }
});
