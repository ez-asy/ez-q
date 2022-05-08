use anyhow::Result;
use fred::client::RedisClient;

use super::redis_keys;

pub async fn remove_message_from_processing_set(
    redis_client: &RedisClient,
    id: &str,
) -> Result<bool> {
    let number_of_removed_fields: u8 = redis_client.hdel(redis_keys::processing(), id).await?;
    if number_of_removed_fields == 0 {
        return Ok(false);
    }
    Ok(true)
}
