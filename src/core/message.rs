use anyhow::{anyhow, Result};
use bytes_utils::Str;
use chrono::Utc;
use fred::types::RedisValue;
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};
use uuid::Uuid;

#[derive(Serialize)]
pub struct Message {
    pub id: String,
    pub state: MessageState,
    pub queue_name: String,
    pub created_at: i64,
    pub retry_count: u8,
    pub content: serde_json::Value,
}

impl Message {
    pub fn new(queue_name: &str, content: serde_json::Value) -> Message {
        let id = Uuid::new_v4().to_string();
        let created_at = Utc::now().timestamp_millis();
        let retry_count = 0;
        let message_state = MessageState::Waiting;

        Message {
            id,
            created_at,
            retry_count,
            queue_name: queue_name.to_string(),
            state: message_state,
            content,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageState {
    Waiting,
    Processing,
    Completed,
    Failed,
}

impl Into<RedisValue> for MessageState {
    fn into(self) -> RedisValue {
        match self {
            MessageState::Waiting => RedisValue::String(Str::from("Waiting")),
            MessageState::Processing => RedisValue::String(Str::from("Processing")),
            MessageState::Completed => RedisValue::String(Str::from("Completed")),
            MessageState::Failed => RedisValue::String(Str::from("Failed")),
        }
    }
}

impl FromStr for MessageState {
    fn from_str(input: &str) -> Result<MessageState> {
        match input {
            "Waiting" => Ok(MessageState::Waiting),
            "Processing" => Ok(MessageState::Processing),
            "Completed" => Ok(MessageState::Completed),
            "Failed" => Ok(MessageState::Failed),
            _ => Err(anyhow!("Error creating MessageState from string.")),
        }
    }

    type Err = anyhow::Error;
}

impl fmt::Display for MessageState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MessageState::Waiting => write!(f, "Waiting"),
            MessageState::Processing => write!(f, "Processing"),
            MessageState::Completed => write!(f, "Completed"),
            MessageState::Failed => write!(f, "Failed"),
        }
    }
}

// impl RedisResponse for MessageState {
//     fn from_values(
//         values: Vec<fred::types::RedisValue>,
//     ) -> Result<Vec<Self>, fred::prelude::RedisError> {
//         values.into_iter().map(|v| Self::from_value(v)).collect()
//     }

//     fn from_value(value: fred::types::RedisValue) -> Result<Self, fred::prelude::RedisError> {
//         Self::from_str(&value.as_str().unwrap())
//             .map_err(|_| (RedisError::new(fred::error::RedisErrorKind::Unknown, "")))
//     }
// }
