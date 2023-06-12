use std::{collections::HashSet, pin::Pin};

use futures::Future;
use strum::IntoEnumIterator;
use uuid::Uuid;

use super::{BadgeEval, BadgeMetric, BadgeValue};
use crate::database::{
    managers::trades::TradesManager,
    projections::badge::{EnumValue, TakerBadge},
};

pub struct MetricInput {
    pub trades_manager: TradesManager,
    pub user_id: Uuid,
}

impl BadgeMetric for TakerBadge {
    type MetricInput = MetricInput;
    fn metric<'a>(
    ) -> fn(&'a Self::MetricInput) -> Pin<Box<dyn Future<Output = sqlx::Result<i64>> + Send + 'a>>
    {
        |input: &Self::MetricInput| {
            Box::pin(async {
                input
                    .trades_manager
                    .get_num_taker_trades_for_user(input.user_id)
                    .await
            })
        }
    }
}

impl BadgeEval<Self> for TakerBadge {
    fn eval(metric: i64) -> HashSet<Self> {
        Self::iter()
            .filter(|f| f.to_value().value <= metric)
            .collect()
    }
}

impl BadgeValue for TakerBadge {
    fn to_value(&self) -> &'static EnumValue {
        match self {
            Self::FirstTaker => &EnumValue {
                name: "",
                description: "",
                value: 1,
            },
            Self::TakerBeginner => &EnumValue {
                name: "",
                description: "",
                value: 1,
            },
            Self::TakerBandit => &EnumValue {
                name: "",
                description: "",
                value: 1,
            },
            Self::TakerBoss => &EnumValue {
                name: "",
                description: "",
                value: 1,
            },
            Self::TakerBaron => &EnumValue {
                name: "",
                description: "",
                value: 1,
            },
            Self::TakerBandwagon => &EnumValue {
                name: "",
                description: "",
                value: 1,
            },
            Self::TakerBehemoth => &EnumValue {
                name: "",
                description: "",
                value: 1,
            },
        }
    }
}
