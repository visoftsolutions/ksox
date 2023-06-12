use std::{collections::HashSet, future::Future, pin::Pin};

use crate::database::projections::badge::{BadgeName, EnumValue};

pub mod maker;
pub mod taker;
pub mod trade;
pub mod transfer;
pub mod valut;

pub trait BadgeMetric {
    type MetricInput;
    fn metric<'a>(
    ) -> fn(&'a Self::MetricInput) -> Pin<Box<dyn Future<Output = sqlx::Result<i64>> + Send + 'a>>;
}

pub trait BadgeValue {
    fn to_value(&self) -> &'static EnumValue;
}

pub trait BadgeEval<T> {
    fn eval(metric: i64) -> HashSet<T>;
}

impl BadgeValue for BadgeName {
    fn to_value(&self) -> &'static EnumValue {
        match self {
            Self::ValutBadge(badge) => badge.to_value(),
            Self::TradeBadge(badge) => badge.to_value(),
            Self::TransferBadge(badge) => badge.to_value(),
            Self::MakerBadge(badge) => badge.to_value(),
            Self::TakerBadge(badge) => badge.to_value(),
        }
    }
}
