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

#[derive(Debug, Deserialize)]
pub struct ListMessagesData {
    #[serde(rename = "listMessages")]
    pub list_messages: Vec<MessageData>,
}
