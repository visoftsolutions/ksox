use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
struct Network {
    id: Uuid,
    name: String,
}
