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
    mutation UpdateUserStatus($userId: ID!, $status: String!) {
        updateUserStatus(userId: $userId, status: $status) {
            userId
            username
            status
            lastSeen
        }
    }
"#;

pub const CREATE_USER_MUTATION: &str = r#"
    mutation CreateUser($userId: ID!, $username: String!, $email: String!) {
        createUser(userId: $userId, username: $username, email: $email) {
            userId
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
    pub user_id: String,
    pub status: String,
}

#[derive(Serialize)]
pub struct CreateUserVariables {
    pub user_id: String,
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
    pub update_user_status: User,
}

#[derive(Deserialize)]
pub struct CreateUserResponse {
    pub create_user: User,
}
