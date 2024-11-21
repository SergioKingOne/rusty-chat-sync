use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageData {
    #[serde(rename = "messageId")]
    pub message_id: String,
    pub content: String,
    pub sender: String,
    pub timestamp: f64,
    pub chat_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationData {
    #[serde(rename = "chatId")]
    pub chat_id: String,
    #[serde(rename = "otherUser")]
    pub other_user: UserData,
    #[serde(rename = "lastMessage")]
    pub last_message: Option<MessageData>,
    #[serde(rename = "unreadCount")]
    pub unread_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserData {
    pub username: String,
    pub email: String,
    #[serde(rename = "createdAt")]
    pub created_at: f64,
    #[serde(rename = "lastSeen")]
    pub last_seen: Option<f64>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct GraphQLRequest<T> {
    pub query: String,
    pub variables: T,
    pub operation_name: String,
}

#[derive(Debug, Deserialize)]
pub struct GraphQLResponse<T> {
    pub data: Option<T>,
    pub errors: Option<Vec<GraphQLError>>,
}

#[derive(Debug, Deserialize)]
pub struct GraphQLError {
    pub message: String,
    pub locations: Option<Vec<ErrorLocation>>,
    pub path: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct ErrorLocation {
    pub line: i32,
    pub column: i32,
}
