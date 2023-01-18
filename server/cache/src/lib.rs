use anyhow::{Ok, Result};
pub use redis;

pub fn get_client() -> Result<redis::Client> {
    Ok(redis::Client::open(
        std::env::var("REDIS_URL").unwrap_or_default().as_str(),
    )?)
}
