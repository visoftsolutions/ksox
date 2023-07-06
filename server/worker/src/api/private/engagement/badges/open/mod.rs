mod sse;

use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};
use engagement::{
    database::{
        managers::{trades::TradesManager, transfers::TransfersManager, valuts::ValutsManager},
        projections::badge::{
            BadgeFamily, MakerBadge, TakerBadge, TradeBadge, TransferBadge, ValutBadge,
        },
    },
    engagement_engine::badges::{
        maker, taker, trade, transfer, valut, BadgeEval, BadgeMetric, BadgeValue,
    },
};
use fraction::Fraction;
use num_bigint::BigInt;
use serde::{Deserialize, Serialize};

use crate::{
    api::{auth::models::UserId, AppError},
    models::AppState,
};

pub fn router(app_state: &AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/sse", get(sse::root))
        .with_state(app_state.clone())
}

#[derive(Debug, Deserialize)]
pub struct Request {
    pub badge_family: BadgeFamily,
}

#[derive(Debug, Serialize)]
pub struct Response {
    name: String,
    description: String,
    progress: Fraction,
}

pub async fn root(
    State(state): State<AppState>,
    user_id: UserId,
    Query(request): Query<Request>,
) -> Result<Json<Option<Response>>, AppError> {
    Ok(Json(match request.badge_family {
        BadgeFamily::ValutBadge => {
            let mut metric = (ValutBadge::metric())(&valut::MetricInput {
                valuts_manager: &ValutsManager::new(state.database),
                user_id: *user_id,
            })
            .await?;

            ValutBadge::eval_open(&mut metric)
                .map(|f| f.to_value())
                .map(|f| Response {
                    name: f.name.to_string(),
                    description: f.description.to_string(),
                    progress: Fraction::from_raw((BigInt::from(metric), BigInt::from(f.value)))
                        .unwrap_or_default(),
                })
        }
        BadgeFamily::TradeBadge => {
            let mut metric = (TradeBadge::metric())(&trade::MetricInput {
                trades_manager: &TradesManager::new(state.database),
                user_id: *user_id,
            })
            .await?;

            TradeBadge::eval_open(&mut metric)
                .map(|f| f.to_value())
                .map(|f| Response {
                    name: f.name.to_string(),
                    description: f.description.to_string(),
                    progress: Fraction::from_raw((BigInt::from(metric), BigInt::from(f.value)))
                        .unwrap_or_default(),
                })
        }
        BadgeFamily::TransferBadge => {
            let mut metric = (TransferBadge::metric())(&transfer::MetricInput {
                transfers_manager: &TransfersManager::new(state.database),
                user_id: *user_id,
            })
            .await?;

            TransferBadge::eval_open(&mut metric)
                .map(|f| f.to_value())
                .map(|f| Response {
                    name: f.name.to_string(),
                    description: f.description.to_string(),
                    progress: Fraction::from_raw((BigInt::from(metric), BigInt::from(f.value)))
                        .unwrap_or_default(),
                })
        }
        BadgeFamily::TakerBadge => {
            let mut metric = (TakerBadge::metric())(&taker::MetricInput {
                trades_manager: &TradesManager::new(state.database),
                user_id: *user_id,
            })
            .await?;

            TakerBadge::eval_open(&mut metric)
                .map(|f| f.to_value())
                .map(|f| Response {
                    name: f.name.to_string(),
                    description: f.description.to_string(),
                    progress: Fraction::from_raw((BigInt::from(metric), BigInt::from(f.value)))
                        .unwrap_or_default(),
                })
        }
        BadgeFamily::MakerBadge => {
            let mut metric = (MakerBadge::metric())(&maker::MetricInput {
                trades_manager: &TradesManager::new(state.database),
                user_id: *user_id,
            })
            .await?;

            MakerBadge::eval_open(&mut metric)
                .map(|f| f.to_value())
                .map(|f| Response {
                    name: f.name.to_string(),
                    description: f.description.to_string(),
                    progress: Fraction::from_raw((BigInt::from(metric), BigInt::from(f.value)))
                        .unwrap_or_default(),
                })
        }
    }))
}
