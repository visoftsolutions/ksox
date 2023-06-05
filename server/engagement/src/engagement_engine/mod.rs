use sqlx::{PgPool};

use crate::base::engagement_server::Engagement;

pub struct EngagementEngine {
    database: PgPool,
}

impl EngagementEngine {
    pub fn new(database: PgPool) -> Self {
        Self { database }
    }
}

#[tonic::async_trait]
impl Engagement for EngagementEngine {
}