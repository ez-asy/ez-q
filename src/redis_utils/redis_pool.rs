use std::env;

use anyhow::Result;
use fred::pool::RedisPool;
use fred::prelude::*;

pub async fn create_redis_pool() -> Result<RedisPool> {
    let redis_url_env = env::var("REDIS_URL").unwrap_or(String::from("redis://localhost:6379"));
    let config = RedisConfig::from_url(&redis_url_env).unwrap_or(RedisConfig::default());
    let policy = ReconnectPolicy::default();
    let pool = RedisPool::new(config, 6)?;
    let _ = pool.connect(Some(policy));
    let _ = pool.wait_for_connect().await?;
    Ok(pool)
}
