use crate::graphql::types::MessageData;
use crate::models::user::User;
use serde::{Deserialize, Serialize};

pub const CREATE_MESSAGE_MUTATION: &str = r#"
    mutation CreateMessage($content: String!, $authorId: ID!) {
        createMessage(content: $content, authorId: $authorId) {
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

pub const UPDATE_USER_STATUS_MUTATION: &str = r#"
    mutation UpdateUserStatus($username: String!, $status: String!) {
        updateUserStatus(username: $username, status: $status) {
            username
            status
            lastSeen
        }
    }
"#;

pub const CREATE_USER_MUTATION: &str = r#"
    mutation CreateUser($username: String!, $email: String!) {
        createUser(username: $username, email: $email) {
            username
            email
            createdAt
            lastSeen
            status
        }
    }
"#;

#[derive(Serialize)]
pub struct CreateMessageVariables {
    pub content: String,
    pub author_id: String,
}

#[derive(Serialize)]
pub struct UpdateUserStatusVariables {
    pub username: String,
    pub status: String,
}

#[derive(Serialize)]
pub struct CreateUserVariables {
    pub username: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct CreateMessageResponse {
    #[serde(rename = "createMessage")]
    pub create_message: MessageData,
}

#[derive(Deserialize)]
pub struct UpdateUserStatusResponse {
    #[serde(rename = "updateUserStatus")]
    pub update_user_status: User,
}

#[derive(Deserialize)]
pub struct CreateUserResponse {
    pub create_user: User,
}
