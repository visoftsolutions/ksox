use axum::{
    extract::{Query, State},
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    api::AppError, database::projections::asset::Asset, models::AppState,
    recognition::AssetPairRecognitionResult,
};

#[derive(Deserialize)]
pub struct Request {
    pub input: String,
}

#[derive(Debug, Serialize)]
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
