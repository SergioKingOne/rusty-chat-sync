use crate::models::message::{Message, MessageStatus, MessageType};
use chrono::{Local, TimeZone};
use web_sys::HtmlElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MessageListProps {
    pub messages: Vec<Message>,
}

#[function_component(MessageList)]
pub fn message_list(props: &MessageListProps) -> Html {
    let list_ref = use_node_ref();
    let new_messages = use_state(|| 0);
    let auto_scroll = use_state(|| true);

    // Scroll handling
    {
        let list_ref = list_ref.clone();
        let auto_scroll = auto_scroll.clone();
        let messages_len = props.messages.len();
        let new_messages = new_messages.clone();

        use_effect_with(messages_len, move |_| {
            if let Some(list) = list_ref.cast::<HtmlElement>() {
                if *auto_scroll {
                    list.scroll_to_with_x_and_y(0.0, list.scroll_height() as f64);
                } else {
                    new_messages.set(*new_messages + 1);
                }
            }
            || ()
        });
    }

    // Scroll event listener
    let onscroll = {
        let list_ref = list_ref.clone();
        let auto_scroll = auto_scroll.clone();

        Callback::from(move |_| {
            if let Some(list) = list_ref.cast::<HtmlElement>() {
                let scroll_top = list.scroll_top() as f64;
                let scroll_height = list.scroll_height() as f64;
                let client_height = list.client_height() as f64;

                // Enable auto-scroll when user scrolls to bottom
                auto_scroll.set(scroll_top + client_height >= scroll_height - 10.0);
            }
        })
    };

    html! {
        <div class="message-list-container">
            <div
                ref={list_ref.clone()}
                class="message-list"
                {onscroll}
            >
                { for props.messages.iter().map(|msg| {
                    let message_class = match msg.message_type {
                        MessageType::System => "system",
                        MessageType::Text => {
                            if msg.author == "User" { "sent" } else { "received" }
                        }
                        MessageType::Error => "error",
                    };

                    html! {
                        <div
                            class={classes!("message-item", message_class)}
                            key={msg.message_id.clone()}
                        >
                            if msg.message_type != MessageType::System {
                                <div class="message-header">
                                    <span class="author">{ &msg.author }</span>
                                    {" • "}
                                    <span class="timestamp">
                                        { format_timestamp(msg.timestamp) }
                                    </span>
                                </div>
                            }
                            <div class="message-content">
                                { &msg.content }
                            </div>
                            if msg.message_type == MessageType::Text {
                                <div
                                    class={classes!(
                                        "message-status",
                                        msg.status.to_string().to_lowercase()
                                    )}
                                >
                                    { get_status_icon(&msg.status) }
                                </div>
                            }
                        </div>
                    }
                }) }
            </div>

            // New messages indicator
            if *new_messages > 0 && !*auto_scroll {
                <div
                    class="new-messages-indicator"
                    onclick={
                        let list_ref = list_ref.clone();
                        let new_messages = new_messages.clone();
                        let auto_scroll = auto_scroll.clone();
                        Callback::from(move |_| {
                            if let Some(list) = list_ref.cast::<HtmlElement>() {
                                list.scroll_to_with_x_and_y(0.0, list.scroll_height() as f64);
                                new_messages.set(0);
                                auto_scroll.set(true);
                            }
                        })
                    }
                >
                    { format!("{} new messages ↓", *new_messages) }
                </div>
            }
        </div>
    }
}

fn get_status_icon(status: &MessageStatus) -> &'static str {
    match status {
        MessageStatus::Sending => "⋯",
        MessageStatus::Sent => "✓",
        MessageStatus::Failed => "!",
    }
}

fn format_timestamp(timestamp: f64) -> String {
    // Convert JavaScript timestamp (milliseconds) to DateTime
    let datetime = Local.timestamp_millis_opt(timestamp as i64).unwrap();

    // Format time as "12:34 PM"
    datetime.format("%I:%M %p").to_string()
}
