mod display;
mod external;
mod sse;

use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};
use chrono::{DateTime, Utc};
use evm::address::Address;
use fraction::Fraction;
use serde::{Deserialize, Serialize};
use tokio_stream::StreamExt;
use uuid::Uuid;

use crate::{
    api::{auth::models::UserId, AppError, Pagination},
    database::projections::transfer::{ExtendedTransfer, Transfer},
    models::AppState,
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DisplayTransferDirection {
    Income,
    Outcome,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DisplayTransfer {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub user_id: Uuid,
    pub user_address: Address,
    pub user_name: Option<String>,
    pub other_user_id: Uuid,
    pub other_user_address: Address,
    pub other_user_name: Option<String>,
    pub asset_id: Uuid,
    pub asset_icon_path: String,
    pub asset_name: String,
    pub asset_symbol: String,
    pub amount: Fraction,
    pub fee: Fraction,
    pub direction: DisplayTransferDirection,
}
impl DisplayTransfer {
    fn from_extended_transfer(id: Uuid, transfer: ExtendedTransfer) -> Self {
        let (
            user_id,
            user_address,
            user_name,
            other_user_id,
            other_user_address,
            other_user_name,
            direction,
        ) = if id == transfer.from_user_id {
            (
                transfer.from_user_id,
                transfer.from_user_address,
                transfer.from_user_name,
                transfer.to_user_id,
                transfer.to_user_address,
                transfer.to_user_name,
                DisplayTransferDirection::Outcome,
            )
        } else {
            (
                transfer.to_user_id,
                transfer.to_user_address,
                transfer.to_user_name,
                transfer.from_user_id,
                transfer.from_user_address,
                transfer.from_user_name,
                DisplayTransferDirection::Income,
            )
        };
        DisplayTransfer {
            id: transfer.id,
            created_at: transfer.created_at,
            user_id,
            user_address,
            user_name,
            other_user_id,
            other_user_address,
            other_user_name,
            asset_id: transfer.asset_id,
            asset_icon_path: transfer.asset_icon_path,
            asset_name: transfer.asset_name,
            asset_symbol: transfer.asset_symbol,
            amount: transfer.amount,
            fee: transfer.fee,
            direction,
        }
    }
}

pub fn router(app_state: &AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/sse", get(sse::root))
        .with_state(app_state.clone())
        .nest("/external", external::router(app_state))
        .nest("/display", display::router(app_state))
}

pub async fn root(
    State(state): State<AppState>,
    user_id: UserId,
    Query(params): Query<Pagination>,
) -> Result<Json<Vec<Transfer>>, AppError> {
    let mut stream = state
        .transfers_manager
        .get_for_user_id(*user_id, params.limit, params.offset);
    let mut vec = Vec::<Transfer>::new();
    while let Some(res) = stream.next().await {
        vec.push(res?);
    }
    Ok(Json(vec))
}
