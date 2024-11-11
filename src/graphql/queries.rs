use crate::graphql::types::MessageData;
use serde::Deserialize;

pub const LIST_MESSAGES_QUERY: &str = r#"
    query ListMessages {
        listMessages {
            messageId
            content
            author
            timestamp
        }
    }
"#;

#[derive(Deserialize)]
pub struct ListMessagesResponse {
    pub list_messages: Vec<MessageData>,
}
