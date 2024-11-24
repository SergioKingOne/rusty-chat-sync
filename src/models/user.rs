use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub username: String,
    pub email: String,
    #[serde(rename = "createdAt")]
    pub created_at: f64,
    #[serde(rename = "lastSeen")]
    pub last_seen: Option<f64>,
    pub status: Option<String>,
}

impl User {
    pub fn new(username: String, email: String) -> Self {
        Self {
            username,
            email,
            created_at: js_sys::Date::now(),
            last_seen: None,
            status: None,
        }
    }
}
