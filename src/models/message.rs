use crate::graphql::types::MessageData;
use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Display)]
pub enum MessageStatus {
    #[strum(serialize = "sending")]
    Sending,
    #[strum(serialize = "sent")]
    Sent,
    #[strum(serialize = "failed")]
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MessageType {
    Text,
    System,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Message {
    pub message_id: String,
    pub content: String,
    pub author: String,
    pub timestamp: f64,
    pub status: MessageStatus,
    pub message_type: MessageType,
}

impl Message {
    pub fn new_text(content: String, author: String) -> Self {
        Self {
            message_id: uuid::Uuid::new_v4().to_string(),
            content,
            author,
            timestamp: js_sys::Date::now(),
            status: MessageStatus::Sending,
            message_type: MessageType::Text,
        }
    }

    pub fn new_system(content: String) -> Self {
        Self {
            message_id: uuid::Uuid::new_v4().to_string(),
            content,
            author: "System".to_string(),
            timestamp: js_sys::Date::now(),
            status: MessageStatus::Sent,
            message_type: MessageType::System,
        }
    }

    pub fn from_message_data(data: MessageData) -> Self {
        Self {
            message_id: data.message_id,
            content: data.content,
            author: data.author,
            timestamp: data.timestamp,
            status: MessageStatus::Sent,
            message_type: MessageType::Text,
        }
    }
}
