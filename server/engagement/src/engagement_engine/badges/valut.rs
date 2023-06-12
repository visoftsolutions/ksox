use std::{collections::HashSet, future::Future, pin::Pin};

use strum::IntoEnumIterator;
use uuid::Uuid;

use super::{BadgeEval, BadgeMetric, BadgeValue};
use crate::database::{
    managers::valuts::ValutsManager,
    projections::badge::{EnumValue, ValutBadge},
};

pub struct MetricInput {
    pub valuts_manager: ValutsManager,
    pub user_id: Uuid,
}

impl BadgeMetric for ValutBadge {
    type MetricInput = MetricInput;
    fn metric<'a>(
    ) -> fn(&'a Self::MetricInput) -> Pin<Box<dyn Future<Output = sqlx::Result<i64>> + Send + 'a>>
    {
        |input: &Self::MetricInput| {
            Box::pin(async {
                input
                    .valuts_manager
                    .get_num_of_nonzero_for_user(input.user_id)
                    .await
            })
        }
    }
}

impl BadgeEval<Self> for ValutBadge {
    fn eval(metric: i64) -> HashSet<Self> {
        Self::iter()
            .filter(|f| f.to_value().value <= metric)
            .collect()
    }
}

impl BadgeValue for ValutBadge {
    fn to_value(&self) -> &'static EnumValue {
        match self {
            Self::FirstToken => &EnumValue {
                name: "",
                description: "",
                value: 1,
            },
            Self::TokenTourist => &EnumValue {
                name: "",
                description: "",
                value: 1,
            },
            Self::TokenCollector => &EnumValue {
                name: "",
                description: "",
                value: 1,
            },
            Self::TokenTamer => &EnumValue {
                name: "",
                description: "",
                value: 1,
            },
            Self::TokenHoarder => &EnumValue {
                name: "",
                description: "",
                value: 1,
            },
            Self::TokenDiversifier => &EnumValue {
                name: "",
                description: "",
                value: 1,
            },
            Self::TokenMagnate => &EnumValue {
                name: "",
                description: "",
                value: 1,
            },
            Self::TokenOverlord => &EnumValue {
                name: "",
                description: "",
                value: 1,
            },
            Self::TokenTitan => &EnumValue {
                name: "",
                description: "",
                value: 1,
            },
        }
    }
}
