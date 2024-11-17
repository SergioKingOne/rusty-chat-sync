use crate::graphql::types::MessageData;
use serde::Deserialize;

pub const ON_CREATE_MESSAGE_SUBSCRIPTION: &str = r#"
    subscription OnCreateMessage {
        onCreateMessage {
            messageId
            content
            author
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
    #[serde(rename = "author")]
    pub author: String,
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "messageId")]
    pub message_id: String,
    #[serde(rename = "timestamp")]
    pub timestamp: f64,
}

impl OnCreateMessageData {
    pub fn into_message_data(self) -> MessageData {
        MessageData {
            author: self.author,
            content: self.content,
            message_id: self.message_id,
            timestamp: self.timestamp,
        }
    }
}
