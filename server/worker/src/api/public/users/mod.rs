pub mod search_user;

use axum::{extract::State, routing::get, Json, Router};
use futures::StreamExt;

use crate::{api::AppError, database::projections::user::User, models::AppState};

pub fn router(app_state: &AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/search_user", get(search_user::root))
        .with_state(app_state.clone())
}

// Return all users that exist in db
pub async fn root(State(state): State<AppState>) -> Result<Json<Vec<User>>, AppError> {
    let mut stream = state.users_manager.get_all();
    let mut vec = Vec::<User>::new();
    while let Some(res) = stream.next().await {
        vec.push(res?);
    }
    Ok(Json(vec))
}
