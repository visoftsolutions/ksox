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
    engagement_engine::badges::{maker, taker, trade, transfer, valut, BadgeMetric},
};
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
    score: i64,
}

impl Response {
    pub fn new(score: i64) -> Self {
        Self { score }
    }
}

pub async fn root(
    State(state): State<AppState>,
    user_id: UserId,
    Query(request): Query<Request>,
) -> Result<Json<Response>, AppError> {
    Ok(Json(Response::new(match request.badge_family {
        BadgeFamily::ValutBadge => {
            (ValutBadge::metric())(&valut::MetricInput {
                valuts_manager: ValutsManager::new(state.database),
                user_id: *user_id,
            })
            .await?
        }
        BadgeFamily::TradeBadge => {
            (TradeBadge::metric())(&trade::MetricInput {
                trades_manager: TradesManager::new(state.database),
                user_id: *user_id,
            })
            .await?
        }
        BadgeFamily::TransferBadge => {
            (TransferBadge::metric())(&transfer::MetricInput {
                transfers_manager: TransfersManager::new(state.database),
                user_id: *user_id,
            })
            .await?
        }
        BadgeFamily::TakerBadge => {
            (TakerBadge::metric())(&taker::MetricInput {
                trades_manager: TradesManager::new(state.database),
                user_id: *user_id,
            })
            .await?
        }
        BadgeFamily::MakerBadge => {
            (MakerBadge::metric())(&maker::MetricInput {
                trades_manager: TradesManager::new(state.database),
                user_id: *user_id,
            })
            .await?
        }
    })))
}
