use serde::Deserialize;

use super::types::MessageData;

pub const ON_CREATE_MESSAGE_SUBSCRIPTION: &str = r#"
    subscription OnCreateMessage($chatId: String!) {
        onCreateMessage(chatId: $chatId) {
            messageId
            content
            sender
            timestamp
            chatId
            status
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
    pub on_create_message: MessageData,
}
