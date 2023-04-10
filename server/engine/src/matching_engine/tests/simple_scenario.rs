use std::str::FromStr;

use database::{sqlx::PgPool, types::Volume};
use uuid::Uuid;

use crate::matching_engine::{models::MatchingEngineRequest, MatchingEngine};

#[tokio::test]
async fn simple_scenario() {
    /* case:
    two assets:
        a: b89ac651-e3ef-4902-9549-f3d29b582233 with m_fee 11/10000 t_fee 17/10000 bitcoin BTC
        b: f96083a1-c5d5-4a1b-809b-0fa4eca0b051 with m_fee 23/1000 t_fee 32/1000 ethereum ETH
    users:
        1: 418704df-53dc-449d-8767-12578c5f0d61 address: 0x3acadfb15e991e8403d2fe3e75ee4782b88cf5b1
        2: e106a9b0-8b7a-40f8-92e3-ce17af265f8d address: 0x3acadfb15e991e8403d2fe3e75ee4782b88cf5b2
    users valuts:
        1: a = 10007 b = 59
        2: a = 101 b = 703
     */
    let asset_a_id = Uuid::from_str("b89ac651-e3ef-4902-9549-f3d29b582233").unwrap();
    let asset_b_id = Uuid::from_str("f96083a1-c5d5-4a1b-809b-0fa4eca0b051").unwrap();
    let user_1_id = Uuid::from_str("418704df-53dc-449d-8767-12578c5f0d61").unwrap();
    let user_2_id = Uuid::from_str("e106a9b0-8b7a-40f8-92e3-ce17af265f8d").unwrap();
    let database = PgPool::connect(std::env::var("DATABASE_URL").unwrap_or_default().as_str())
        .await
        .unwrap();

    let me = MatchingEngine::new(database);

    me.execute_request(MatchingEngineRequest {
        user_id: user_2_id,
        quote_asset_id: asset_b_id,
        base_asset_id: asset_a_id,
        quote_asset_volume: Volume::from(400),
        base_asset_volume: Volume::from(72),
    })
    .await
    .unwrap();

    me.execute_request(MatchingEngineRequest {
        user_id: user_1_id,
        quote_asset_id: asset_a_id,
        base_asset_id: asset_b_id,
        quote_asset_volume: Volume::from(81),
        base_asset_volume: Volume::from(390),
    })
    .await
    .unwrap();
}
