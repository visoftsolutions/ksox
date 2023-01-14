use crate::api::auth::models::UserId;

pub async fn root(user_id: UserId) -> String {
    format!("balance endpoint, Hello {user_id}")
}
