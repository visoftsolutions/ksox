use std::str::FromStr;

use chrono::Utc;
use futures::StreamExt;
use num_bigint::BigInt;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    managers::spot::{
        candlesticks::CandlestickManager, orders::OrdersManager, trades::TradesManager,
        valuts::ValutsManager,
    },
    projections::spot::candlestick::Candlestick,
    traits::TableManager,
    types::{CandlestickType, PriceLevel, Volume},
};

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
// async fn valuts_get_and_subscribe_for_user() {
//     let database = PgPool::connect(std::env::var("DATABASE_URL").unwrap_or_default().as_str())
//         .await
//         .unwrap();

//     let user_id = Uuid::from_str("ead19fc2-9652-444d-8d3c-5256ae80a210").unwrap();
//     let valuts_manager = ValutsManager::new(database);

//     let mut stream = valuts_manager.subscribe_for_user(user_id).await.unwrap();
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

// #[tokio::test]
// async fn insert_metadata() {
//     let database = PgPool::connect(std::env::var("DATABASE_URL").unwrap_or_default().as_str())
//         .await
//         .unwrap();

//     let quote_asset_id = Uuid::from_str("5864a1b9-4ae1-424f-bdb4-f644cb359463").unwrap();
//     let base_asset_id = Uuid::from_str("7a99f052-d941-4dcc-aabd-6695c24deed5").unwrap();

//     let candlestick_metadata_manager = CandlestickMetadataManager::new(database);
//     let metadata_id = Uuid::from_str("17c89368-3ec9-428b-8c5e-18d753fdcc49").unwrap();
//     let candlestick_metadata = CandlestickMetadata {
//         id: metadata_id,
//         quote_asset_id: quote_asset_id,
//         base_asset_id: base_asset_id,
//         kind: CandlestickType::Interval,
//         span: 10,
//     };

//     candlestick_metadata_manager
//         .insert(candlestick_metadata)
//         .await
//         .unwrap();
// }

// #[tokio::test]
// async fn insert_fraction() {
//     let database = PgPool::connect(std::env::var("DATABASE_URL").unwrap_or_default().as_str())
//         .await
//         .unwrap();

//     let metadata_id = Uuid::from_str("17c89368-3ec9-428b-8c5e-18d753fdcc49").unwrap();

//     let candlestick_manager = CandlestickManager::new(database);

//     let candlestick = Candlestick {
//         id: Uuid::from_str("aaf2c482-0e1d-472f-9093-b49002c7d17a").unwrap(),
//         metadata: metadata_id,
//         topen: Utc::now(),
//         tclose: Utc::now(),
//         span: 10,
//         open: TryFrom::try_from((BigInt::from(10), BigInt::from(100))).unwrap(),
//         high: TryFrom::try_from((BigInt::from(10), BigInt::from(100))).unwrap(),
//         low: TryFrom::try_from((BigInt::from(10), BigInt::from(100))).unwrap(),
//         close: TryFrom::try_from((BigInt::from(10), BigInt::from(100))).unwrap(),
//         taker_quote_volume: Volume::from(1),
//         taker_base_volume: Volume::from(1),
//         maker_quote_volume: Volume::from(1),
//         maker_base_volume: Volume::from(1),
//     };

//     candlestick_manager.insert(candlestick).await.unwrap();

//     let row = candlestick_manager
//         .get_by_id(Uuid::from_str("aaf2c482-0e1d-472f-9093-b49002c7d17a").unwrap())
//         .await;

//     println!("{:#?}", row);
// }
