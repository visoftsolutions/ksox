use chrono::{DateTime, Utc};
use sqlx::{postgres::types::PgMoney, types::Uuid};

pub struct Trade {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub taker_id: Uuid,
    pub order_id: Uuid,
    pub taker_quote_volume: PgMoney,
    pub maker_quote_volume: PgMoney,
    pub taker_base_volume: PgMoney,
    pub maker_base_volume: PgMoney,
}
