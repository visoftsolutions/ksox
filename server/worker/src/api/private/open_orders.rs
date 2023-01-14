use crate::api::auth::models::UserId;

pub async fn root(user_id: UserId) -> String {
    format!("open_orders endpoint, Hello {user_id}")
}
