use std::pin::Pin;

use futures::Future;
use uuid::Uuid;

use super::{BadgeEval, BadgeMetric, BadgeValue};
use crate::database::{
    managers::valuts::ValutsManager,
    projections::badge::{EnumValue, ValutBadge},
};

pub struct MetricInput<'a> {
    pub valuts_manager: &'a ValutsManager,
    pub user_id: Uuid,
}

impl<'b> BadgeMetric<'b> for ValutBadge {
    type MetricInput = MetricInput<'b>;
    fn metric<'a>(
    ) -> fn(&'a Self::MetricInput) -> Pin<Box<dyn Future<Output = sqlx::Result<i64>> + Send + 'a>>
    {
        |input: &Self::MetricInput| {
            Box::pin(
                input
                    .valuts_manager
                    .get_num_of_nonzero_for_user(input.user_id),
            )
        }
    }
}

impl BadgeEval<Self> for ValutBadge {}

impl BadgeValue for ValutBadge {
    fn to_value(&self) -> &'static EnumValue {
        match self {
            Self::FirstToken => &EnumValue {
                name: "First Token",
                description: "Badge for having first token",
                value: 1,
            },
            Self::TokenTourist => &EnumValue {
                name: "Token Tourist",
                description: "Badge for having five tokens",
                value: 5,
            },
            Self::TokenCollector => &EnumValue {
                name: "Token Collector",
                description: "Badge for having ten tokens",
                value: 10,
            },
            Self::TokenTamer => &EnumValue {
                name: "Token Tamer",
                description: "Badge for having twenty five tokens",
                value: 25,
            },
            Self::TokenHoarder => &EnumValue {
                name: "Token Hoarder",
                description: "Badge for having fifty tokens",
                value: 50,
            },
            Self::TokenDiversifier => &EnumValue {
                name: "Token Diversifier",
                description: "Badge for having seventy five tokens",
                value: 75,
            },
            Self::TokenMagnate => &EnumValue {
                name: "Token Magnate",
                description: "Badge for having hundred tokens",
                value: 100,
            },
            Self::TokenOverlord => &EnumValue {
                name: "Token Overlord",
                description: "Badge for having five hundred tokens",
                value: 500,
            },
            Self::TokenTitan => &EnumValue {
                name: "Token Titan",
                description: "Badge for having thousand tokens",
                value: 1000,
            },
        }
    }
}
