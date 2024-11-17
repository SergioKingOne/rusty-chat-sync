use crate::models::message::{Message, MessageStatus};
use std::rc::Rc;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct ChatState {
    pub messages: Vec<Message>,
    pub is_loading: bool,
    pub error: Option<String>,
}

pub enum ChatAction {
    AddMessage(Message),
    UpdateMessage(String, Message),
    UpdateMessageStatus(String, MessageStatus),
    SetError(String),
    ClearError,
    SetLoading(bool),
    SetMessages(Vec<Message>),
}

impl Reducible for ChatState {
    type Action = ChatAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut next_state = (*self).clone();

        match action {
            ChatAction::AddMessage(msg) => {
                let exists = next_state
                    .messages
                    .iter()
                    .any(|m| m.message_id == msg.message_id);

                if !exists {
                    next_state.messages.push(msg);
                    next_state
                        .messages
                        .sort_by(|a, b| a.timestamp.partial_cmp(&b.timestamp).unwrap());
                }
            }
            ChatAction::UpdateMessageStatus(id, status) => {
                if let Some(msg) = next_state.messages.iter_mut().find(|m| m.message_id == id) {
                    msg.status = status;
                }
            }
            ChatAction::SetError(error) => {
                next_state.error = Some(error);
            }
            ChatAction::ClearError => {
                next_state.error = None;
            }
            ChatAction::SetLoading(is_loading) => {
                next_state.is_loading = is_loading;
            }
            ChatAction::SetMessages(messages) => {
                next_state.messages = messages;
            }
            ChatAction::UpdateMessage(id, new_message) => {
                if let Some(msg) = next_state.messages.iter_mut().find(|m| m.message_id == id) {
                    *msg = new_message;
                    msg.status = MessageStatus::Sent;
                }
                next_state
                    .messages
                    .sort_by(|a, b| a.timestamp.partial_cmp(&b.timestamp).unwrap());
            }
        }

        let new_state = Rc::new(next_state);
        new_state
    }
}
