use serde::{Deserialize, Serialize};

#[derive(sqlx::Type, Debug, Serialize, Deserialize, Clone, PartialEq)]
#[sqlx(type_name = "candlestick_type", rename_all = "lowercase")]
pub enum CandlestickType {
    Interval,
    Tick,
}
