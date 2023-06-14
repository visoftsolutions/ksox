use std::pin::Pin;

use futures::Future;
use uuid::Uuid;

use super::{BadgeEval, BadgeMetric, BadgeValue};
use crate::database::{
    managers::transfers::TransfersManager,
    projections::badge::{EnumValue, TransferBadge},
};

pub struct MetricInput<'a> {
    pub transfers_manager: &'a TransfersManager,
    pub user_id: Uuid,
}

impl<'b> BadgeMetric<'b> for TransferBadge {
    type MetricInput = MetricInput<'b>;
    fn metric<'a>(
    ) -> fn(&'a Self::MetricInput) -> Pin<Box<dyn Future<Output = sqlx::Result<i64>> + Send + 'a>>
    {
        |input: &Self::MetricInput| {
            Box::pin(async {
                let maker_transfers = input
                    .transfers_manager
                    .get_num_maker_transfers_for_user(input.user_id)
                    .await?;
                let taker_transfers = input
                    .transfers_manager
                    .get_num_taker_transfers_for_user(input.user_id)
                    .await?;
                Ok(maker_transfers + taker_transfers)
            })
        }
    }
}

impl BadgeEval<Self> for TransferBadge {}

impl BadgeValue for TransferBadge {
    fn to_value(&self) -> &'static EnumValue {
        match self {
            Self::FirstTransfer => &EnumValue {
                name: "First Transfer",
                description: "Badge for performing first transfer",
                value: 1,
            },
            Self::TransferRookie => &EnumValue {
                name: "Transfer Rookie",
                description: "Badge for performing ten transfers",
                value: 10,
            },
            Self::TransferTrooper => &EnumValue {
                name: "Transfer Trooper",
                description: "Badge for performing one hundred transfers",
                value: 100,
            },
            Self::TransferCourier => &EnumValue {
                name: "Transfer Courier",
                description: "Badge for performing five hundred transfers",
                value: 500,
            },
            Self::TransferMagician => &EnumValue {
                name: "Transfer Magician",
                description: "Badge for performing thousand transfers",
                value: 1000,
            },
            Self::TransferWizard => &EnumValue {
                name: "Transfer Wizard",
                description: "Badge for performing five thousand transfers",
                value: 5000,
            },
            Self::TransferTerminator => &EnumValue {
                name: "Transfer Terminator",
                description: "Badge for performing ten thousand transfers",
                value: 10000,
            },
            Self::TransferLegend => &EnumValue {
                name: "Transfer Legend",
                description: "Badge for performing fifty thousand transfers",
                value: 50000,
            },
            Self::TransferTitan => &EnumValue {
                name: "Transfer Titan",
                description: "Badge for performing hundred thousand transfers",
                value: 100000,
            },
        }
    }
}
