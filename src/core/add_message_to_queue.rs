use anyhow::Result;
use fred::client::RedisClient;

use super::redis_keys;

pub async fn add_message_to_queue(conn: &RedisClient, id: &str, queue_name: &str) -> Result<bool> {
    let _add_message_res: u8 = conn.lpush(redis_keys::queued(&queue_name), id).await?;

    Ok(true)
}
