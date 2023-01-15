use anyhow::{Ok, Result};

pub fn get_redis_client() -> Result<redis::Client> {
    Ok(redis::Client::open(
        std::env::var("REDIS_URL").unwrap_or_default().as_str(),
    )?)
}
