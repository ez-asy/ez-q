use std::{collections::HashMap, str::FromStr};

use anyhow::{anyhow, Result};
use fred::client::RedisClient;

use crate::core::message::Message;

use super::{message::MessageState, redis_keys};

pub async fn _get_message_data(client: &RedisClient, msg_id: &str) -> Result<Message> {
    let message_data: HashMap<String, String> = client.hgetall(redis_keys::message(msg_id)).await?;

    let content = match message_data.get("content") {
        Some(value) => value,
        None => return Err(anyhow!("Data in message not as expected.")),
    };
    let id = match message_data.get("id") {
        Some(value) => value,
        None => return Err(anyhow!("Data in message not as expected.")),
    };
    let state = match message_data.get("state") {
        Some(value) => value,
        None => return Err(anyhow!("Data in message not as expected.")),
    };
    let queue_name = match message_data.get("queue_name") {
        Some(value) => value,
        None => return Err(anyhow!("Data in message not as expected.")),
    };
    let created_at = match message_data.get("created_at") {
        Some(value) => value,
        None => return Err(anyhow!("Data in message not as expected.")),
    };
    let retry_count = match message_data.get("retry_count") {
        Some(value) => value,
        None => return Err(anyhow!("Data in message not as expected.")),
    };

    let message: Message = Message {
        content: serde_json::from_str(content)?,
        state: MessageState::from_str(state)?,
        id: id.to_owned(),
        queue_name: queue_name.to_owned(),
        created_at: created_at.parse::<i64>()?,
        retry_count: retry_count.parse::<u8>()?,
    };

    Ok(message)
}
