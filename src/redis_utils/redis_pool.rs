use anyhow::Result;
use fred::pool::RedisPool;
use fred::prelude::*;

pub async fn create_redis_pool() -> Result<RedisPool> {
    let config = RedisConfig::default();
    let policy = ReconnectPolicy::default();
    let pool = RedisPool::new(config, 6)?;
    let _ = pool.connect(Some(policy));
    let _ = pool.wait_for_connect().await?;
    Ok(pool)
}
