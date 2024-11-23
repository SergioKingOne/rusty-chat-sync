use crate::graphql::types::MessageData;
use crate::models::user::User;
use serde::Deserialize;

use super::types::ConversationData;

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

pub const GET_CONVERSATION_QUERY: &str = r#"
    query GetConversation($otherUsername: String!) {
        getConversation(otherUsername: $otherUsername) {
            messageId
            content
            sender
            timestamp
            chatId
            status
        }
    }
"#;

pub const LIST_CONVERSATIONS_QUERY: &str = r#"
    query ListConversations {
        listConversations {
            chatId
            otherUser {
                username
                email
                status
                lastSeen
            }
            lastMessage {
                messageId
                content
                sender
                timestamp
            }
            unreadCount
        }
    }
"#;

pub const LIST_USERS_QUERY: &str = r#"
    query ListUsers {
        listUsers {
            username
            email
            createdAt
            status
            lastSeen
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

#[derive(Debug, Deserialize)]
pub struct GetConversationResponse {
    pub get_conversation: Vec<MessageData>,
}

#[derive(Debug, Deserialize)]
pub struct ListConversationsResponse {
    pub list_conversations: Vec<ConversationData>,
}

#[derive(Debug, Deserialize)]
pub struct ListUsersResponse {
    #[serde(rename = "listUsers")]
    pub list_users: Vec<User>,
}
