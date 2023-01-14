use sqlx::{postgres::Postgres, Pool};

use crate::projections::user::User;

struct UsersManager {
    database: Pool<Postgres>,
}

impl UsersManager {
    pub fn new(database: Pool<Postgres>) -> Self {
        UsersManager { database }
    }

    pub async fn get_all(&self) -> Vec<User> {
        let query = sqlx::query_as!(
            User,
            "SELECT * FROM users;"
        );

        Vec::new()
    }
}
