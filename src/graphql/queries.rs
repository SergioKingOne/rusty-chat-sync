use crate::graphql::types::MessageData;
use crate::models::user::User;
use serde::Deserialize;

pub const LIST_MESSAGES_QUERY: &str = r#"
    query ListMessages {
        listMessages {
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

pub const GET_USER_QUERY: &str = r#"
    query GetUser($userId: ID!) {
        getUser(userId: $userId) {
            userId
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
            userId
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
