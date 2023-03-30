use axum::{
    extract::{Query, State},
    Json,
};
use database::{projections::spot::asset::Asset, sqlx::types::Uuid};
use serde::{Deserialize, Serialize};

use crate::{api::AppError, models::AppState, recognition::AssetPairRecognitionResult};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub input: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetResponse {
    pub id: Uuid,
    pub name: String,
    pub symbol: String,
}

impl From<Asset> for AssetResponse {
    fn from(value: Asset) -> Self {
        Self {
            id: value.id,
            name: value.name,
            symbol: value.symbol,
        }
    }
}

// Send string phrase and return vector of suggestions sorted by most "relevant"
pub async fn root(
    State(state): State<AppState>,
    Query(params): Query<Request>,
) -> Result<Json<Vec<AssetPairRecognitionResult>>, AppError> {
    Ok(Json(
        state
            .assets_pair_recognition
            .recognize(&params.input)
            .await?,
    ))
}
