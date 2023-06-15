use std::{future::Future, pin::Pin};

use strum::IntoEnumIterator;

use crate::database::projections::badge::{BadgeName, EnumValue};

pub mod maker;
pub mod taker;
pub mod trade;
pub mod transfer;
pub mod valut;

pub trait BadgeMetric<'b> {
    type MetricInput;
    fn metric<'a>(
    ) -> fn(&'a Self::MetricInput) -> Pin<Box<dyn Future<Output = sqlx::Result<i64>> + Send + 'a>>;
}

pub trait BadgeValue {
    fn to_value(&self) -> &'static EnumValue;
}

pub trait BadgeEval<T>
where
    T: IntoEnumIterator + BadgeValue + std::hash::Hash + Eq,
{
    fn eval_recived(metric: &mut i64) -> Vec<T> {
        T::iter()
            .map_while(|f| {
                let element = f.to_value();
                if *metric >= element.value {
                    *metric -= element.value;
                    Some(f)
                } else {
                    None
                }
            })
            .collect()
    }
    fn eval_open(metric: &mut i64) -> Option<T> {
        T::iter().find_map(|f| {
            let element = f.to_value();
            if *metric < element.value {
                Some(f)
            } else {
                *metric -= element.value;
                None
            }
        })
    }
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
