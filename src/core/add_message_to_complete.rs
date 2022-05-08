use anyhow::Result;
use chrono::Utc;
use fred::{client::RedisClient, types::RedisValue};

use super::redis_keys;

pub async fn add_message_to_complete_set(redis_client: &RedisClient, id: &str) -> Result<bool> {
    let now = Utc::now().timestamp_millis();
    let _number_of_members_added = redis_client
        .hset(redis_keys::complete(), (id, RedisValue::Integer(now)))
        .await?;
    Ok(true)
}
