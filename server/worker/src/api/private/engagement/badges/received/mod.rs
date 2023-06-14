mod sse;

use axum::{extract::State, routing::get, Json, Router};
use engagement::{
    database::{
        managers::{trades::TradesManager, transfers::TransfersManager, valuts::ValutsManager},
        projections::badge::{MakerBadge, TakerBadge, TradeBadge, TransferBadge, ValutBadge},
    },
    engagement_engine::badges::{
        maker, taker, trade, transfer, valut, BadgeEval, BadgeMetric, BadgeValue,
    },
};
use serde::Serialize;

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

#[derive(Debug, Serialize)]
pub struct Response {
    name: String,
    description: String,
}

impl Response {
    pub fn new(name: String, description: String) -> Self {
        Self { name, description }
    }
}

pub async fn root(
    State(state): State<AppState>,
    user_id: UserId,
) -> Result<Json<Vec<Response>>, AppError> {
    Ok(Json(
        ValutBadge::eval_recived(
            &mut (ValutBadge::metric())(&valut::MetricInput {
                valuts_manager: &ValutsManager::new(state.database.clone()),
                user_id: *user_id,
            })
            .await?,
        )
        .into_iter()
        .map(|f| f.to_value())
        .chain(
            TradeBadge::eval_recived(
                &mut (TradeBadge::metric())(&trade::MetricInput {
                    trades_manager: &TradesManager::new(state.database.clone()),
                    user_id: *user_id,
                })
                .await?,
            )
            .into_iter()
            .map(|f| f.to_value()),
        )
        .chain(
            TransferBadge::eval_recived(
                &mut (TransferBadge::metric())(&transfer::MetricInput {
                    transfers_manager: &TransfersManager::new(state.database.clone()),
                    user_id: *user_id,
                })
                .await?,
            )
            .into_iter()
            .map(|f| f.to_value()),
        )
        .chain(
            TakerBadge::eval_recived(
                &mut (TakerBadge::metric())(&taker::MetricInput {
                    trades_manager: &TradesManager::new(state.database.clone()),
                    user_id: *user_id,
                })
                .await?,
            )
            .into_iter()
            .map(|f| f.to_value()),
        )
        .chain(
            MakerBadge::eval_recived(
                &mut (MakerBadge::metric())(&maker::MetricInput {
                    trades_manager: &TradesManager::new(state.database),
                    user_id: *user_id,
                })
                .await?,
            )
            .into_iter()
            .map(|f| f.to_value()),
        )
        .map(|f| Response {
            name: f.name.to_string(),
            description: f.description.to_string(),
        })
        .collect(),
    ))
}
