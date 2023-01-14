use axum::response;

pub async fn submit() -> response::Result<String> {
    Ok("you called submit!".into())
}
