use crate::graphql::types::MessageData;
use serde::{Deserialize, Serialize};

pub const CREATE_MESSAGE_MUTATION: &str = r#"
    mutation CreateMessage($content: String!, $author: String!) {
        createMessage(content: $content, author: $author) {
            messageId
            content
            author
            timestamp
        }
    }
"#;

#[derive(Serialize)]
pub struct CreateMessageVariables {
    pub content: String,
    pub author: String,
}

#[derive(Deserialize)]
pub struct CreateMessageResponse {
    #[serde(rename = "createMessage")]
    pub create_message: MessageData,
}
