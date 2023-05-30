use axum::{
    extract::{Query, State},
    Json,
};
use serde::Deserialize;

use crate::{api::AppError, models::AppState, recognition::user::UserRecognitionResult};

#[derive(Deserialize)]
pub struct Request {
    pub input: String,
}

// Send string phrase and return vector of suggestions sorted by most "relevant"
pub async fn root(
    State(state): State<AppState>,
    Query(params): Query<Request>,
) -> Result<Json<Vec<UserRecognitionResult>>, AppError> {
    Ok(Json(state.user_recognition.recognize(&params.input).await?))
}
