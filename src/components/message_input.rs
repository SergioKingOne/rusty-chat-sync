use crate::models::message::Message;
use uuid::Uuid;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MessageInputProps {
    pub on_send: Callback<Message>,
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

    let onclick = {
        let content = content.clone();
        let on_send = props.on_send.clone();
        Callback::from(move |_| {
            if !content.is_empty() {
                let message = Message {
                    messageId: Uuid::new_v4().to_string(),
                    content: (*content).clone(),
                    author: "User".to_string(), // Replace with authenticated user
                    timestamp: js_sys::Date::now(),
                };
                on_send.emit(message);
                content.set(String::new());
            }
        })
    };

    html! {
        <div>
            <input type="text" value={(*content).clone()} oninput={oninput} placeholder="Type your message..." />
            <button onclick={onclick}>{ "Send" }</button>
        </div>
    }
}
