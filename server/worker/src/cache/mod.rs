pub fn get_client() -> redis::RedisResult<redis::Client> {
    redis::Client::open(std::env::var("REDIS_URL").unwrap_or_default().as_str())
}
