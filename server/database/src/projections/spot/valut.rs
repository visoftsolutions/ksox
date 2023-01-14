use sqlx::types::Uuid;
use ethereum_types::U256;

pub struct Valut {
    id: Uuid,
    user_id: Uuid,
    asset_id: Uuid,
    balance: U256,
}
