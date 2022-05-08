use anyhow::Result;
use fred::client::RedisClient;

use super::redis_keys;

pub async fn get_queue_names(conn: &RedisClient) -> Result<Vec<String>> {
    let queue_names: Vec<String> = conn.smembers(redis_keys::queue_list()).await?;
    Ok(queue_names)
}
