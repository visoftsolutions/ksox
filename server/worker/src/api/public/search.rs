use axum::{extract::State, Json};
use database::{projections::spot::asset::Asset, sqlx::types::Uuid};
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};

use crate::{api::AppError, models::AppState};

#[derive(Deserialize)]
pub struct Request {
    pub input: String,
}

#[derive(Serialize)]
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
    Json(payload): Json<Request>,
) -> Result<Json<Vec<(OrderedFloat<f64>, (AssetResponse, AssetResponse))>>, AppError> {
    Ok(Json(
        state
            .assets_pair_recognition
            .recognize(&payload.input)
            .await?,
    ))
}