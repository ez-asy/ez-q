use anyhow::Result;
use fred::pool::StaticRedisPool;
use fred::prelude::*;

pub async fn create_redis_pool() -> Result<StaticRedisPool> {
    let config = RedisConfig::default();
    let policy = ReconnectPolicy::default();
    let pool = StaticRedisPool::new(config, 6)?;
    let _ = pool.connect(Some(policy));
    let _ = pool.wait_for_connect().await?;
    Ok(pool)
}
