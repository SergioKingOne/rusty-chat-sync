use crate::models::message::Message;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MessageListProps {
    pub messages: Vec<Message>,
}

#[function_component(MessageList)]
pub fn message_list(props: &MessageListProps) -> Html {
    html! {
        <ul>
            { for props.messages.iter().map(|msg| html! {
                <li class="message-item" key="{msg.messageId.clone()}">
                    <div class="message-content">
                        <strong>{ &msg.author }</strong>{": "}{ &msg.content }
                    </div>
                </li>
            }) }
        </ul>
    }
}
