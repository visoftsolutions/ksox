use std::pin::Pin;

use futures::Future;
use uuid::Uuid;

use super::{BadgeEval, BadgeMetric, BadgeValue};
use crate::database::{
    managers::trades::TradesManager,
    projections::badge::{EnumValue, TradeBadge},
};

pub struct MetricInput<'a> {
    pub trades_manager: &'a TradesManager,
    pub user_id: Uuid,
}

impl<'b> BadgeMetric<'b> for TradeBadge {
    type MetricInput = MetricInput<'b>;
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

impl BadgeEval<Self> for TradeBadge {}

impl BadgeValue for TradeBadge {
    fn to_value(&self) -> &'static EnumValue {
        match self {
            Self::FirstTrade => &EnumValue {
                name: "First Trade",
                description: "Badge for performing first trade",
                value: 1,
            },
            Self::TradeNovice => &EnumValue {
                name: "Trade Novice",
                description: "Badge for performing ten trades",
                value: 10,
            },
            Self::TradeTornado => &EnumValue {
                name: "Trade Tornado",
                description: "Badge for performing one hundred trades",
                value: 100,
            },
            Self::TradeTyrant => &EnumValue {
                name: "Trade Tyrant",
                description: "Badge for performing five hundred trades",
                value: 500,
            },
            Self::TradeMogul => &EnumValue {
                name: "Trade Mogul",
                description: "Badge for performing thousand trades",
                value: 1000,
            },
            Self::TradeMagnate => &EnumValue {
                name: "Trade Magnate",
                description: "Badge for performing five thousand trades",
                value: 5000,
            },
            Self::TradeOverlord => &EnumValue {
                name: "Trade Overlord",
                description: "Badge for performing ten thousand trades",
                value: 10000,
            },
            Self::TradeLegend => &EnumValue {
                name: "Trade Legend",
                description: "Badge for performing fifty thousand trades",
                value: 50000,
            },
            Self::TradeTitan => &EnumValue {
                name: "Trade Titan",
                description: "Badge for performing hundred thousand trades",
                value: 100000,
            },
        }
    }
}
