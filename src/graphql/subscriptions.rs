use crate::models::user::User;
use serde::Deserialize;

use super::types::MessageData;

pub const ON_CREATE_MESSAGE_SUBSCRIPTION: &str = r#"
    subscription OnCreateMessage {
        onCreateMessage {
            messageId
            content
            author {
                userId
                username
                email
                createdAt
                lastSeen
                status
            }
            timestamp
        }
    }
"#;

#[derive(Debug, Deserialize)]
pub struct SubscriptionPayload {
    pub data: SubscriptionData,
}

#[derive(Debug, Deserialize)]
pub struct SubscriptionData {
    #[serde(rename = "onCreateMessage")]
    pub on_create_message: OnCreateMessageData,
}

#[derive(Debug, Deserialize)]
pub struct OnCreateMessageData {
    #[serde(rename = "messageId")]
    pub message_id: String,
    #[serde(rename = "content")]
    pub content: String,
    pub author: User,
    #[serde(rename = "timestamp")]
    pub timestamp: f64,
}

impl OnCreateMessageData {
    pub fn into_message_data(self) -> MessageData {
        MessageData {
            message_id: self.message_id,
            content: self.content,
            author: self.author,
            timestamp: self.timestamp,
        }
    }
}
