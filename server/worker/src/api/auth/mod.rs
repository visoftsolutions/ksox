pub mod utils;

use super::AppError;
use axum::{routing::get, routing::post, Json, Router};
use serde::{Deserialize, Serialize};

pub fn router() -> Router {
    Router::new()
        .route("/", get(generate_nonce))
        .route("/", post(validate_signature))
}

#[derive(Debug, Serialize, Deserialize)]
struct GenerateNonceRequest {
    address: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GenerateNonceResponse {
    nonce: String,
    expiration: usize,
}

async fn generate_nonce(
    params: Json<GenerateNonceRequest>,
) -> Result<Json<GenerateNonceResponse>, AppError> {
    Err(AppError(anyhow::Error::msg("not implemented")))
}

#[derive(Debug, Serialize, Deserialize)]
struct ValidateSignatureRequest {
    address: String,
    signature: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ValidateSignatureResponse {
    token: String,
    expiration: usize,
}
async fn validate_signature(
    params: Json<ValidateSignatureRequest>,
) -> Result<Json<ValidateSignatureResponse>, AppError> {
    Err(AppError(anyhow::Error::msg("not implemented")))
}
