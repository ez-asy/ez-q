use anyhow::Result;
use fred::client::RedisClient;

use super::redis_keys;

pub async fn get_message_content(client: &RedisClient, msg_id: &str) -> Result<serde_json::Value> {
    let message_content: String = client.hget(redis_keys::message(msg_id), "content").await?;
    let message_content_json = serde_json::from_str(&message_content)?;
    Ok(message_content_json)
}
