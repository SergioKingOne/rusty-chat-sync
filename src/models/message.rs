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
    pub sender: String,
    pub timestamp: f64,
    pub status: MessageStatus,
    pub message_type: MessageType,
    pub chat_id: String,
}

impl Message {
    pub fn new_text(content: String, sender: String, receiver: String) -> Self {
        let mut users = vec![sender.clone(), receiver];
        users.sort();
        let chat_id = format!("CHAT#{}#{}", users[0], users[1]);

        Self {
            message_id: uuid::Uuid::new_v4().to_string(),
            content,
            sender,
            timestamp: js_sys::Date::now(),
            status: MessageStatus::Sending,
            message_type: MessageType::Text,
            chat_id,
        }
    }

    pub fn new_system(content: String) -> Self {
        Self {
            message_id: uuid::Uuid::new_v4().to_string(),
            content,
            sender: "system".to_string(),
            timestamp: js_sys::Date::now(),
            status: MessageStatus::Sent,
            message_type: MessageType::System,
            chat_id: "SYSTEM".to_string(),
        }
    }

    pub fn from_message_data(data: MessageData) -> Self {
        Self {
            message_id: data.message_id,
            content: data.content,
            sender: data.sender,
            timestamp: data.timestamp,
            status: MessageStatus::Sent,
            message_type: MessageType::Text,
            chat_id: data.chat_id,
        }
    }
}
