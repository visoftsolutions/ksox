use anyhow::{Ok, Result};

pub fn get_redis_client() -> Result<redis::Client> {
    let uri = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://ksox-redis/".to_string());
    let client = redis::Client::open(uri)?;
    Ok(client)
}
