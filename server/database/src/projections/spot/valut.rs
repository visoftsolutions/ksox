use ethereum_types::U256;
use sqlx::types::Uuid;

pub struct Valut {
    id: Uuid,
    user_id: Uuid,
    asset_id: Uuid,
    balance: U256,
}
