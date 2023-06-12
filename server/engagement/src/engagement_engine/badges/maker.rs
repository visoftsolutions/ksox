use std::{collections::HashSet, pin::Pin};

use futures::Future;
use strum::IntoEnumIterator;
use uuid::Uuid;

use super::{BadgeEval, BadgeMetric, BadgeValue};
use crate::database::{
    managers::trades::TradesManager,
    projections::badge::{EnumValue, MakerBadge},
};

pub struct MetricInput {
    pub trades_manager: TradesManager,
    pub user_id: Uuid,
}

impl BadgeMetric for MakerBadge {
    type MetricInput = MetricInput;
    fn metric<'a>(
    ) -> fn(&'a Self::MetricInput) -> Pin<Box<dyn Future<Output = sqlx::Result<i64>> + Send + 'a>>
    {
        |input: &Self::MetricInput| {
            Box::pin(async {
                input
                    .trades_manager
                    .get_num_maker_trades_for_user(input.user_id)
                    .await
            })
        }
    }
}

impl BadgeEval<Self> for MakerBadge {
    fn eval(metric: i64) -> HashSet<Self> {
        Self::iter()
            .filter(|f| f.to_value().value <= metric)
            .collect()
    }
}

impl BadgeValue for MakerBadge {
    fn to_value(&self) -> &'static EnumValue {
        match self {
            Self::FirstMaker => &EnumValue {
                name: "",
                description: "",
                value: 1,
            },
            Self::MakerApprentice => &EnumValue {
                name: "",
                description: "",
                value: 1,
            },
            Self::MakerAficionado => &EnumValue {
                name: "",
                description: "",
                value: 1,
            },
            Self::MakerAvenger => &EnumValue {
                name: "",
                description: "",
                value: 1,
            },
            Self::MakerAce => &EnumValue {
                name: "",
                description: "",
                value: 1,
            },
            Self::MakerAvalanche => &EnumValue {
                name: "",
                description: "",
                value: 1,
            },
            Self::MakerOverlord => &EnumValue {
                name: "",
                description: "",
                value: 1,
            },
        }
    }
}
