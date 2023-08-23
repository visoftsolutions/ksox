use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct AssignedBadge {
    id: Uuid,
    user_id: Uuid,
    badge_id: Uuid,
    created_at: DateTime<Utc>,
    last_modification_at: DateTime<Utc>,
}
