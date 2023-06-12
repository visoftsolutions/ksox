use std::{collections::HashSet, pin::Pin};

use futures::Future;
use strum::IntoEnumIterator;
use uuid::Uuid;

use super::{BadgeEval, BadgeMetric, BadgeValue};
use crate::database::{
    managers::trades::TradesManager,
    projections::badge::{EnumValue, TradeBadge},
};

pub struct MetricInput {
    pub trades_manager: TradesManager,
    pub user_id: Uuid,
}

impl BadgeMetric for TradeBadge {
    type MetricInput = MetricInput;
    fn metric<'a>(
    ) -> fn(&'a Self::MetricInput) -> Pin<Box<dyn Future<Output = sqlx::Result<i64>> + Send + 'a>>
    {
        |input: &Self::MetricInput| {
            Box::pin(async {
                let maker_trades = input
                    .trades_manager
                    .get_num_maker_trades_for_user(input.user_id)
                    .await?;
                let taker_trades = input
                    .trades_manager
                    .get_num_taker_trades_for_user(input.user_id)
                    .await?;
                Ok(maker_trades + taker_trades)
            })
        }
    }
}

impl BadgeEval<Self> for TradeBadge {
    fn eval(metric: i64) -> HashSet<Self> {
        Self::iter()
            .filter(|f| f.to_value().value <= metric)
            .collect()
    }
}

impl BadgeValue for TradeBadge {
    fn to_value(&self) -> &'static EnumValue {
        match self {
            Self::FirstTrade => &EnumValue {
                name: "",
                description: "",
                value: 1,
            },
            Self::TradeNovice => &EnumValue {
                name: "",
                description: "",
                value: 1,
            },
            Self::TradeTornado => &EnumValue {
                name: "",
                description: "",
                value: 1,
            },
            Self::TradeTyrant => &EnumValue {
                name: "",
                description: "",
                value: 1,
            },
            Self::TradeMogul => &EnumValue {
                name: "",
                description: "",
                value: 1,
            },
            Self::TradeMagnate => &EnumValue {
                name: "",
                description: "",
                value: 1,
            },
            Self::TradeOverlord => &EnumValue {
                name: "",
                description: "",
                value: 1,
            },
            Self::TradeLegend => &EnumValue {
                name: "",
                description: "",
                value: 1,
            },
            Self::TradeTitan => &EnumValue {
                name: "",
                description: "",
                value: 1,
            },
        }
    }
}
