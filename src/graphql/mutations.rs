use crate::graphql::types::MessageData;
use crate::models::user::User;
use serde::{Deserialize, Serialize};

pub const CREATE_MESSAGE_MUTATION: &str = r#"
    mutation CreateMessage($content: String!, $receiverUsername: String!) {
        createMessage(content: $content, receiverUsername: $receiverUsername) {
            messageId
            content
            sender
            timestamp
            chatId
            status
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
    pub receiver_username: String,
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
    #[serde(rename = "createUser")]
    pub create_user: User,
}
