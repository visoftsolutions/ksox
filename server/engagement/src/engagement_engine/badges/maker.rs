use std::pin::Pin;

use futures::Future;
use uuid::Uuid;

use super::{BadgeEval, BadgeMetric, BadgeValue};
use crate::database::{
    managers::trades::TradesManager,
    projections::badge::{EnumValue, MakerBadge},
};

pub struct MetricInput<'a> {
    pub trades_manager: &'a TradesManager,
    pub user_id: Uuid,
}

impl<'b> BadgeMetric<'b> for MakerBadge {
    type MetricInput = MetricInput<'b>;
    fn metric<'a>(
    ) -> fn(&'a Self::MetricInput) -> Pin<Box<dyn Future<Output = sqlx::Result<i64>> + Send + 'a>>
    {
        |input: &Self::MetricInput| {
            Box::pin(
                input
                    .trades_manager
                    .get_num_maker_trades_for_user(input.user_id),
            )
        }
    }
}

impl BadgeEval<Self> for MakerBadge {}

impl BadgeValue for MakerBadge {
    fn to_value(&self) -> &'static EnumValue {
        match self {
            Self::FirstMaker => &EnumValue {
                name: "First Maker",
                description: "Badge for performing first maker trade",
                value: 1,
            },
            Self::MakerApprentice => &EnumValue {
                name: "Maker Apprentice",
                description: "Badge for performing ten maker trades",
                value: 10,
            },
            Self::MakerAficionado => &EnumValue {
                name: "Maker Aficionado",
                description: "Badge for performing hundred maker trades",
                value: 100,
            },
            Self::MakerAvenger => &EnumValue {
                name: "Maker Avenger",
                description: "Badge for performing five hundred maker trades",
                value: 500,
            },
            Self::MakerAce => &EnumValue {
                name: "Maker Ace",
                description: "Badge for performing thousand maker trades",
                value: 1000,
            },
            Self::MakerAvalanche => &EnumValue {
                name: "Maker Avalanche",
                description: "Badge for performing five thousand maker trades",
                value: 5000,
            },
            Self::MakerOverlord => &EnumValue {
                name: "Maker Overlord",
                description: "Badge for performing ten thousand maker trades",
                value: 10000,
            },
        }
    }
}
