use std::pin::Pin;

use futures::Future;
use uuid::Uuid;

use super::{BadgeEval, BadgeMetric, BadgeValue};
use crate::database::{
    managers::trades::TradesManager,
    projections::badge::{EnumValue, TakerBadge},
};

pub struct MetricInput<'a> {
    pub trades_manager: &'a TradesManager,
    pub user_id: Uuid,
}

impl<'b> BadgeMetric<'b> for TakerBadge {
    type MetricInput = MetricInput<'b>;
    fn metric<'a>(
    ) -> fn(&'a Self::MetricInput) -> Pin<Box<dyn Future<Output = sqlx::Result<i64>> + Send + 'a>>
    {
        |input: &Self::MetricInput| {
            Box::pin(
                input
                    .trades_manager
                    .get_num_taker_trades_for_user(input.user_id),
            )
        }
    }
}

impl BadgeEval<Self> for TakerBadge {}

impl BadgeValue for TakerBadge {
    fn to_value(&self) -> &'static EnumValue {
        match self {
            Self::FirstTaker => &EnumValue {
                name: "First Taker",
                description: "Badge for performing first taker trade",
                value: 1,
            },
            Self::TakerBeginner => &EnumValue {
                name: "Taker Beginner",
                description: "Badge for performing ten taker trades",
                value: 10,
            },
            Self::TakerBandit => &EnumValue {
                name: "Taker Bandit",
                description: "Badge for performing hundred taker trades",
                value: 100,
            },
            Self::TakerBoss => &EnumValue {
                name: "Taker Boss",
                description: "Badge for performing five hundred taker trades",
                value: 500,
            },
            Self::TakerBaron => &EnumValue {
                name: "Taker Baron",
                description: "Badge for performing thousand taker trades",
                value: 1000,
            },
            Self::TakerBandwagon => &EnumValue {
                name: "Taker Bandwagon",
                description: "Badge for performing five thousand taker trades",
                value: 5000,
            },
            Self::TakerBehemoth => &EnumValue {
                name: "Taker Behemoth",
                description: "Badge for performing ten thousand taker trades",
                value: 10000,
            },
        }
    }
}
