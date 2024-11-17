use crate::models::message::{Message, MessageStatus, MessageType};
use chrono::{Local, TimeZone};
use web_sys::HtmlElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MessageListProps {
    pub messages: Vec<Message>,
    pub current_user_id: String,
    pub is_loading: bool,
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

    // Formatting functions can remain inside the component
    fn format_date(timestamp: f64) -> String {
        let now = Local::now();
        let message_date = Local.timestamp_millis_opt(timestamp as i64).unwrap();

        if message_date.date_naive() == now.date_naive() {
            "Today".to_string()
        } else if message_date.date_naive() == now.date_naive().pred_opt().unwrap() {
            "Yesterday".to_string()
        } else {
            message_date.format("%B %d, %Y").to_string()
        }
    }

    html! {
        <div class="message-list-container">
            <div
                ref={list_ref.clone()}
                class="message-list"
                {onscroll}
            >
                {
                    if props.is_loading {
                        (0..5).map(|i| {
                            html! {
                                <div class="message-skeleton" key={i} />
                            }
                        }).collect::<Html>()
                    } else {
                        html! {
                            <div>
                                {
                                    {
                                        let mut current_date = String::new();
                                        props.messages.iter().map(|msg| {
                                            let mut elements = Vec::new();
                                            let msg_date = format_date(msg.timestamp);

                                            if msg_date != current_date {
                                                current_date = msg_date.clone();
                                                elements.push(html! {
                                                    <div class="date-separator">
                                                        <span class="date-text">{ msg_date }</span>
                                                    </div>
                                                });
                                            }

                                            let message_class = match msg.message_type {
                                                MessageType::System => "system",
                                                MessageType::Text => {
                                                    if msg.author == props.current_user_id { "sent" } else { "received" }
                                                }
                                                MessageType::Error => "error",
                                            };

                                            elements.push(html! {
                                                <div
                                                    class={classes!("message-item", message_class)}
                                                    key={msg.message_id.clone()}
                                                >
                                                    {
                                                        if msg.message_type != MessageType::System {
                                                            html! {
                                                                <div class="message-header">
                                                                    <span class="author">{ &msg.author }</span>
                                                                    {" • "}
                                                                    <span class="timestamp">
                                                                        { format_timestamp(msg.timestamp) }
                                                                    </span>
                                                                </div>
                                                            }
                                                        } else {
                                                            html! {}
                                                        }
                                                    }
                                                    <div class="message-content">
                                                        { &msg.content }
                                                    </div>
                                                    {
                                                        if msg.message_type == MessageType::Text {
                                                            html! {
                                                                <div
                                                                    class={classes!(
                                                                        "message-status",
                                                                        msg.status.to_string().to_lowercase()
                                                                    )}
                                                                >
                                                                    { get_status_icon(&msg.status) }
                                                                </div>
                                                            }
                                                        } else {
                                                            html! {}
                                                        }
                                                    }
                                                </div>
                                            });

                                            elements
                                        }).collect::<Vec<_>>().into_iter().flatten().collect::<Html>()
                                    }
                                }
                            </div>
                        }
                    }
                }
            </div>
            {
                if *new_messages > 0 && !*auto_scroll {
                    html! {
                        <div class="new-messages-indicator">
                            { format!("{} new messages ↓", *new_messages) }
                        </div>
                    }
                } else {
                    html! {}
                }
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
