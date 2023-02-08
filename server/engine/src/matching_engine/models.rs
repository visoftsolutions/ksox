use database::{
    projections::spot::{order::Order, trade::Trade},
    sqlx::types::Uuid,
    types::Volume,
};

pub struct MatchingEngineRequest {
    pub user_id: Uuid,
    pub base_asset_id: Uuid,
    pub quote_asset_id: Uuid,
    pub base_asset_volume: Volume,
    pub quote_asset_volume: Volume,
}

pub struct MatchingEngineResponse {
    pub orders: Vec<Order>,
    pub trades: Vec<Trade>,
}

impl MatchingEngineResponse {
    pub fn new() -> MatchingEngineResponse {
        MatchingEngineResponse {
            orders: Vec::new(),
            trades: Vec::new(),
        }
    }
}
