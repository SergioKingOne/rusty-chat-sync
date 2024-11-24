use super::message::Message;
use super::user::User;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Conversation {
    pub chat_id: String,
    pub other_user: User,
    pub last_message: Option<Message>,
    pub unread_count: i32,
}

impl Conversation {
    pub fn new(current_user: &str, other_user: User) -> Self {
        let mut users = vec![current_user.to_string(), other_user.username.clone()];
        users.sort();
        let chat_id = format!("CHAT#{}#{}", users[0], users[1]);

        Self {
            chat_id,
            other_user,
            last_message: None,
            unread_count: 0,
        }
    }
}
