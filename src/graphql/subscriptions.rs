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
pub struct OnCreateMessageData {
    #[serde(rename = "onCreateMessage")]
    pub message: MessageData,
}
