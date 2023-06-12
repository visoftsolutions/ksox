use std::{collections::HashSet, pin::Pin};

use futures::Future;
use strum::IntoEnumIterator;
use uuid::Uuid;

use super::{BadgeEval, BadgeMetric, BadgeValue};
use crate::database::{
    managers::transfers::TransfersManager,
    projections::badge::{EnumValue, TransferBadge},
};

pub struct MetricInput {
    pub transfers_manager: TransfersManager,
    pub user_id: Uuid,
}

impl BadgeMetric for TransferBadge {
    type MetricInput = MetricInput;
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

impl BadgeEval<Self> for TransferBadge {
    fn eval(metric: i64) -> HashSet<Self> {
        Self::iter()
            .filter(|f| f.to_value().value <= metric)
            .collect()
    }
}

impl BadgeValue for TransferBadge {
    fn to_value(&self) -> &'static EnumValue {
        match self {
            Self::FirstTransfer => &EnumValue {
                name: "",
                description: "",
                value: 1,
            },
            Self::TransferRookie => &EnumValue {
                name: "",
                description: "",
                value: 1,
            },
            Self::TransferTrooper => &EnumValue {
                name: "",
                description: "",
                value: 1,
            },
            Self::TransferCourier => &EnumValue {
                name: "",
                description: "",
                value: 1,
            },
            Self::TransferMagician => &EnumValue {
                name: "",
                description: "",
                value: 1,
            },
            Self::TransferWizard => &EnumValue {
                name: "",
                description: "",
                value: 1,
            },
            Self::TransferTerminator => &EnumValue {
                name: "",
                description: "",
                value: 1,
            },
            Self::TransferLegend => &EnumValue {
                name: "",
                description: "",
                value: 1,
            },
            Self::TransferTitan => &EnumValue {
                name: "",
                description: "",
                value: 1,
            },
        }
    }
}
