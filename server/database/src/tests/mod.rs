// use std::str::FromStr;

// use chrono::Utc;
// use futures::StreamExt;
// use num_bigint::BigInt;
// use sqlx::PgPool;
// use uuid::Uuid;

// use crate::{
//     managers::spot::{
//         candlesticks::CandlestickManager, orders::OrdersManager, trades::TradesManager,
//         valuts::ValutsManager,
//     },
//     projections::spot::candlestick::Candlestick,
//     traits::TableManager,
//     types::{CandlestickType, PriceLevel, Volume},
// };

// #[tokio::test]
// async fn trades_get_and_subscribe_for_taker() {
//     let database = PgPool::connect(std::env::var("DATABASE_URL").unwrap_or_default().as_str())
//         .await
//         .unwrap();

//     let taker_id = Uuid::from_str("ead19fc2-9652-444d-8d3c-5256ae80a210").unwrap();
//     let trades_manager = TradesManager::new(database);

//     let mut stream = trades_manager.subscribe_for_taker(taker_id).await.unwrap();
//     while let Some(result) = stream.next().await {
//         match result {
//             Ok(trade) => {
//                 println!("{:#?}", trade);
//             }
//             Err(err) => {
//                 println!("{err}");
//             }
//         }
//     }
// }

// #[tokio::test]
// async fn orders_get_and_subscribe_for_user() {
//     let database = PgPool::connect(std::env::var("DATABASE_URL").unwrap_or_default().as_str())
//         .await
//         .unwrap();

//     let user_id = Uuid::from_str("ead19fc2-9652-444d-8d3c-5256ae80a210").unwrap();
//     let orders_manager = OrdersManager::new(database);

//     let mut stream = orders_manager.subscribe_for_user(user_id).await.unwrap();
//     while let Some(result) = stream.next().await {
//         match result {
//             Ok(trade) => {
//                 println!("{:#?}", trade);
//             }
//             Err(err) => {
//                 println!("{err}");
//             }
//         }
//     }
// }

// #[tokio::test]
// async fn orders_get_and_subscribe_for_asset_pair() {
//     let database = PgPool::connect(std::env::var("DATABASE_URL").unwrap_or_default().as_str())
//         .await
//         .unwrap();

//     let quote_asset_id = Uuid::from_str("5864a1b9-4ae1-424f-bdb4-f644cb359463").unwrap();
//     let base_asset_id = Uuid::from_str("7a99f052-d941-4dcc-aabd-6695c24deed5").unwrap();
//     let trades_manager = TradesManager::new(database);

//     let mut stream = trades_manager
//         .subscribe_for_asset_pair(quote_asset_id, base_asset_id)
//         .await
//         .unwrap();
//     while let Some(result) = stream.next().await {
//         match result {
//             Ok(trade) => {
//                 println!("{:#?}", trade);
//             }
//             Err(err) => {
//                 println!("{err}");
//             }
//         }
//     }
// }

// #[tokio::test]
// async fn trades_get_and_subscribe_for_orderbook() {
//     let database = PgPool::connect(std::env::var("DATABASE_URL").unwrap_or_default().as_str())
//         .await
//         .unwrap();

//     let quote_asset_id = Uuid::from_str("5864a1b9-4ae1-424f-bdb4-f644cb359463").unwrap();
//     let base_asset_id = Uuid::from_str("7a99f052-d941-4dcc-aabd-6695c24deed5").unwrap();

//     let orders_manager = OrdersManager::new(database);

//     let mut stream = orders_manager
//         .subscribe_for_orderbook(quote_asset_id, base_asset_id, 5, 5)
//         .await
//         .unwrap();
//     while let Some(result) = stream.next().await {
//         match result {
//             Ok(order) => {
//                 println!("{:#?}", order);
//             }
//             Err(err) => {
//                 println!("{err}");
//             }
//         }
//     }
// }

// #[tokio::test]
// async fn trades_get_orderbook() {
//     let database = PgPool::connect(std::env::var("DATABASE_URL").unwrap_or_default().as_str())
//         .await
//         .unwrap();

//     let quote_asset_id = Uuid::from_str("5864a1b9-4ae1-424f-bdb4-f644cb359463").unwrap();
//     let base_asset_id = Uuid::from_str("7a99f052-d941-4dcc-aabd-6695c24deed5").unwrap();

//     let orders_manager = OrdersManager::new(database);

//     let a = orders_manager
//         .get_orderbook(quote_asset_id, base_asset_id, 1, 5)
//         .map(|f| f.and_then(|e| TryInto::<PriceLevel>::try_into(e)));
//     let vec: Vec<Result<PriceLevel, sqlx::Error>> = a.collect().await;

//     println!("{:#?}", vec);
// }
