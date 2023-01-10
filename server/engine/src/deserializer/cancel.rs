use axum::{
    response
};

pub async fn cancel() -> response::Result<String> {
    Ok("you called cancel!".into())
}
