use sqlx::{postgres::types::PgMoney, types::Uuid};

pub struct Valut {
    pub id: Uuid,
    pub user_id: Uuid,
    pub asset_id: Uuid,
    pub balance: PgMoney,
}
