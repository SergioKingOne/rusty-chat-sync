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
                next_state.messages.push(msg);
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
        }

        Rc::new(next_state)
    }
}
