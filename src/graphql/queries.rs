use crate::graphql::types::MessageData;
use crate::models::user::User;
use serde::Deserialize;

pub const LIST_MESSAGES_QUERY: &str = r#"
    query ListMessages {
        listMessages {
            messageId
            content
            username
            timestamp
        }
    }
"#;

pub const GET_USER_QUERY: &str = r#"
    query GetUser($username: String!) {
        getUser(username: $username) {
            username
            email
            createdAt
            lastSeen
            status
        }
    }
"#;

pub const GET_USER_BY_EMAIL_QUERY: &str = r#"
    query GetUserByEmail($email: String!) {
        getUserByEmail(email: $email) {
            username
            email
            createdAt
            lastSeen
            status
        }
    }
"#;

#[derive(Debug, Deserialize)]
pub struct ListMessagesData {
    #[serde(rename = "listMessages")]
    pub list_messages: Vec<MessageData>,
}

#[derive(Debug, Deserialize)]
pub struct GetUserResponse {
    pub get_user: User,
}

#[derive(Debug, Deserialize)]
pub struct GetUserByEmailResponse {
    pub get_user_by_email: User,
}
