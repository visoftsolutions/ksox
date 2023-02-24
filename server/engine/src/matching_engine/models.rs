use database::{
    projections::spot::{order::Order, trade::Trade},
    sqlx::types::Uuid,
    types::Volume,
};

pub struct MatchingEngineRequest {
    pub user_id: Uuid,
    pub quote_asset_id: Uuid,
    pub base_asset_id: Uuid,
    pub quote_asset_volume: Volume,
    pub base_asset_volume: Volume,
}

#[derive(Debug)]
pub struct MatchingEngineResponse {
    pub order: Option<Order>,
    pub trades: Vec<Trade>,
}

impl std::fmt::Display for MatchingEngineResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MatchingEngineResponse:
        order: {:?}
        trades: {:?}
        ",
            self.order, self.trades
        )
    }
}

impl MatchingEngineResponse {
    pub fn new() -> MatchingEngineResponse {
        MatchingEngineResponse {
            order: None,
            trades: Vec::new(),
        }
    }
}
