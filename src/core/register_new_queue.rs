use anyhow::Result;
use fred::{clients::RedisClient, interfaces::SetsInterface};

use super::redis_keys;

pub async fn register_queue(client: &RedisClient, queue_name: &str) -> Result<bool> {
    let res: u8 = client.sadd(redis_keys::queue_list(), queue_name).await?;
    if res == 0 {
        return Ok(false);
    }
    Ok(true)
}
