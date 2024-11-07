use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Message {
    pub messageId: String,
    pub content: String,
    pub author: String,
    pub timestamp: f64,
}
