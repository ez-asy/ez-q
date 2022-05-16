use anyhow::Result;
use fred::{
    clients::RedisClient,
    interfaces::HashesInterface,
    types::{RedisMap, RedisValue},
};

use super::{message::Message, redis_keys};

pub async fn save_message(client: &RedisClient, message: Message) -> Result<String> {
    let id = message.id;

    let id_as_value: RedisValue = id.clone().into();
    let queue_name: RedisValue = message.queue_name.into();
    let created_at: RedisValue = message.created_at.into();
    let retry_count: RedisValue = message.retry_count.into();
    let content: RedisValue = message.content.to_string().into();
    let state: RedisValue = message.state.into();

    let key_value: RedisMap = RedisMap::try_from(vec![
        ("id", id_as_value),
        ("queue_name", queue_name),
        ("created_at", created_at),
        ("retry_count", retry_count),
        ("content", content),
        ("state", state),
    ])
    .unwrap();

    let _: u8 = client.hset(redis_keys::message(&id), key_value).await?;

    Ok(id)
}
