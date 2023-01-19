use std::ops::Deref;

use axum::{extract::State, Json};
use database::projections::spot::valut::Valut;
use tokio_stream::StreamExt;

use crate::{
    api::{auth::models::UserId, AppError},
    AppState,
};

pub async fn root(
    State(state): State<AppState>,
    user_id: UserId,
) -> Result<Json<Vec<Valut>>, AppError> {
    let mut stream = state.valuts_manager.get_by_user_id(*user_id.deref()).await;
    let mut vec = Vec::<Valut>::new();
    while let Some(res) = stream.next().await {
        vec.push(res?);
    }
    Ok(Json(vec))
}
