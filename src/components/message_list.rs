use crate::models::message::Message;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MessageListProps {
    pub messages: Vec<Message>,
}

#[function_component(MessageList)]
pub fn message_list(props: &MessageListProps) -> Html {
    html! {
        <div class="message-list">
            <ul>
                { for props.messages.iter().map(|msg| html! {
                    <li class="message-item" key={msg.message_id.clone()}>
                        <div class="message-header">
                            <strong>{ &msg.author }</strong>
                            <span class="timestamp">
                                { format_timestamp(msg.timestamp) }
                            </span>
                        </div>
                        <div class="message-content">
                            { &msg.content }
                        </div>
                    </li>
                }) }
            </ul>
        </div>
    }
}

fn format_timestamp(timestamp: f64) -> String {
    // You might want to add the chrono crate for better date formatting
    let date = js_sys::Date::new(&timestamp.into());
    date.to_locale_time_string("en-US").into()
}
