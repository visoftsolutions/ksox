use crate::base::engagement_server::Engagement;
pub mod badges;

#[derive(Debug, Default)]
pub struct EngagementEngine {}

#[tonic::async_trait]
impl Engagement for EngagementEngine {}
