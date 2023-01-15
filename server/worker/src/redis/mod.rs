use anyhow::{Ok, Result};

pub fn get_redis_client() -> Result<redis::Client> {
    Ok(redis::Client::open(std::env!("REDIS_URL"))?)
}
