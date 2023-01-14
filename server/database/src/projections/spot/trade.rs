use chrono::{DateTime, Utc};
use ethereum_types::U256;
use sqlx::types::Uuid;

pub struct Trade {
    id: Uuid,
    created_at: DateTime<Utc>,
    taker_id: Uuid,
    order_id: Uuid,
    taker_quote_volume: U256,
    maker_quote_volume: U256,
    taker_base_volume: U256,
    maker_base_volume: U256,
}
