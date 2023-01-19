use crate::api::auth::models::UserId;

pub async fn root(user_id: UserId) -> String {
    format!("cancel_order endpoint, Hello {user_id}")
}
