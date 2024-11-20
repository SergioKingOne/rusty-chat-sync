use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageData {
    #[serde(rename = "messageId")]
    pub message_id: String,
    pub content: String,
    pub username: String,
    pub timestamp: f64,
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
