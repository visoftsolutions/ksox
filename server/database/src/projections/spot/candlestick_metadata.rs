use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(sqlx::Type, Debug, Serialize, Deserialize, Clone, PartialEq)]
#[sqlx(type_name = "candlestick_type", rename_all = "lowercase")]
pub enum CandlestickType {
    Interval,
    Tick,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CandlestickMetadata {
    pub id: Uuid,
    pub quote_asset_id: Uuid,
    pub base_asset_id: Uuid,
    pub kind: CandlestickType,
    pub span: i32,
}
