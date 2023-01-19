use axum::{extract::State, Json};
use database::projections::spot::order::Order;
use futures::StreamExt;

use crate::{
    api::{auth::models::UserId, AppError},
    AppState,
};

pub async fn root(
    State(state): State<AppState>,
    user_id: UserId,
) -> Result<Json<Vec<Order>>, AppError> {
    let mut stream = state.orders_manager.get_by_user_id(*user_id).await;
    let mut vec = Vec::<Order>::new();
    while let Some(res) = stream.next().await {
        vec.push(res?);
    }
    Ok(Json(vec))
}
