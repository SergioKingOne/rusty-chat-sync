use crate::models::{
    message::{Message, MessageStatus, MessageType},
    user::User,
};
use uuid::Uuid;
use web_sys::KeyboardEvent;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MessageInputProps {
    pub on_send: Callback<Message>,
    pub user_id: String,
}

#[function_component(MessageInput)]
pub fn message_input(props: &MessageInputProps) -> Html {
    let content = use_state(|| String::new());

    let oninput = {
        let content = content.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            content.set(input.value());
        })
    };

    let send_message = {
        let content = content.clone();
        let on_send = props.on_send.clone();
        let username = props.user_id.clone();
        move || {
            if !content.is_empty() {
                let message = Message {
                    message_id: Uuid::new_v4().to_string(),
                    content: (*content).clone(),
                    author: User::new(username.clone(), username.clone(), "".to_string()),
                    status: MessageStatus::Sending,
                    message_type: MessageType::Text,
                    timestamp: js_sys::Date::now(),
                };
                on_send.emit(message);
                content.set(String::new());
            }
        }
    };

    let onkeypress = {
        let send = send_message.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" && !e.shift_key() {
                e.prevent_default();
                send();
            }
        })
    };

    let onclick = {
        let send = send_message;
        Callback::from(move |_| send())
    };

    html! {
        <div class="message-input-container">
            <input
                type="text"
                class="message-input"
                value={(*content).clone()}
                {oninput}
                {onkeypress}
                placeholder="Type a message and press Enter to send..."
                autofocus=true
            />
            <button
                class="send-button"
                {onclick}
                disabled={content.is_empty()}
            >
                { "Send" }
            </button>
        </div>
    }
}
