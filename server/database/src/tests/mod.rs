use core::time;
use std::{str::FromStr, thread::sleep};

use futures::StreamExt;
use sqlx::PgPool;
use uuid::Uuid;

use crate::managers::spot::{orders::OrdersManager, trades::TradesManager, valuts::ValutsManager};

#[tokio::test]
async fn trades_get_subscribe() {
    let database = PgPool::connect(std::env::var("DATABASE_URL").unwrap_or_default().as_str())
        .await
        .unwrap();

    let user_id = Uuid::from_str("ead19fc2-9652-444d-8d3c-5256ae80a210").unwrap();
    let trades_manager = TradesManager::new(database);

    let mut stream = trades_manager.get_and_subscribe(user_id).await.unwrap();
    while let Some(result) = stream.next().await {
        match result {
            Ok(trade) => {
                println!("{:#?}", trade);
            }
            Err(err) => {
                println!("{err}");
            }
        }
    }
    sleep(time::Duration::from_secs(5));
    stream.destroy().await.unwrap();
}

#[tokio::test]
async fn orders_get_subscribe() {
    let database = PgPool::connect(std::env::var("DATABASE_URL").unwrap_or_default().as_str())
        .await
        .unwrap();

    let user_id = Uuid::from_str("ead19fc2-9652-444d-8d3c-5256ae80a210").unwrap();
    let orders_manager = OrdersManager::new(database);

    let mut stream = orders_manager.get_and_subscribe(user_id).await.unwrap();
    while let Some(result) = stream.next().await {
        match result {
            Ok(trade) => {
                println!("{:#?}", trade);
            }
            Err(err) => {
                println!("{err}");
            }
        }
    }
    sleep(time::Duration::from_secs(5));
    stream.destroy().await.unwrap();
}

#[tokio::test]
async fn valuts_get_subscribe() {
    let database = PgPool::connect(std::env::var("DATABASE_URL").unwrap_or_default().as_str())
        .await
        .unwrap();

    let user_id = Uuid::from_str("ead19fc2-9652-444d-8d3c-5256ae80a210").unwrap();
    let valuts_manager = ValutsManager::new(database);

    let mut stream = valuts_manager.get_and_subscribe(user_id).await.unwrap();
    while let Some(result) = stream.next().await {
        match result {
            Ok(trade) => {
                println!("{:#?}", trade);
            }
            Err(err) => {
                println!("{err}");
            }
        }
    }
    sleep(time::Duration::from_secs(5));
    stream.destroy().await.unwrap();
}
