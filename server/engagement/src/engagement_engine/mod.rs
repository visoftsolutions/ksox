use crate::base::engagement_server::Engagement;

pub struct EngagementEngine {}

impl EngagementEngine {
    pub fn new() -> Self {
        Self {}
    }
}

#[tonic::async_trait]
impl Engagement for EngagementEngine {}
