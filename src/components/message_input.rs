use crate::models::message::{Message, MessageStatus, MessageType};
use uuid::Uuid;
use web_sys::KeyboardEvent;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MessageInputProps {
    pub on_send: Callback<Message>,
    #[prop_or_default]
    pub disabled: bool,
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
        move || {
            if !content.is_empty() {
                let message = Message {
                    message_id: Uuid::new_v4().to_string(),
                    content: (*content).clone(),
                    sender: String::new(),
                    status: MessageStatus::Sending,
                    message_type: MessageType::Text,
                    timestamp: js_sys::Date::now(),
                    chat_id: String::new(),
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
                placeholder={
                    if props.disabled {
                        "Select a conversation to start chatting..."
                    } else {
                        "Type a message and press Enter to send..."
                    }
                }
                disabled={props.disabled}
            />
            <button
                class="send-button"
                {onclick}
                disabled={content.is_empty() || props.disabled}
            >
                { "Send" }
            </button>
        </div>
    }
}
