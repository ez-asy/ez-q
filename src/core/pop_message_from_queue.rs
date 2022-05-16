use anyhow::Result;
use fred::{clients::RedisClient, interfaces::ListInterface};

use super::redis_keys;

pub async fn pop_message_from_queue(client: &RedisClient, queue_name: &str) -> Result<String> {
    let popped_message_id: String = client
        .rpop(redis_keys::queued(&queue_name), Some(1))
        .await?;
    Ok(popped_message_id)
}
