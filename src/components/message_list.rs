use crate::models::message::{Message, MessageStatus, MessageType};
use chrono::{Local, TimeZone};
use web_sys::HtmlElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MessageListProps {
    pub messages: Vec<Message>,
    pub current_user_id: String,
    pub is_loading: bool,
    pub on_scroll: Callback<(f64, f64, f64)>,
    pub show_scroll_button: bool,
    pub on_scroll_to_bottom: Callback<MouseEvent>,
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
        let on_scroll = props.on_scroll.clone();
        let new_messages = new_messages.clone();

        Callback::from(move |_| {
            if let Some(list) = list_ref.cast::<HtmlElement>() {
                let scroll_top = list.scroll_top() as f64;
                let scroll_height = list.scroll_height() as f64;
                let client_height = list.client_height() as f64;

                // Check if scrolled to bottom
                let at_bottom = scroll_top + client_height >= scroll_height - 10.0;

                // Reset new messages when scrolled to bottom
                if at_bottom {
                    new_messages.set(0);
                }

                auto_scroll.set(at_bottom);
                on_scroll.emit((scroll_top, scroll_height, client_height));
            }
        })
    };

    // Formatting functions
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

    fn format_time(timestamp: f64) -> String {
        let datetime = Local.timestamp_millis_opt(timestamp as i64).unwrap();
        datetime.format("%I:%M %p").to_string()
    }

    fn should_show_sender(messages: &[Message], index: usize) -> bool {
        if index == 0 {
            return true;
        }
        let current = &messages[index];
        let previous = &messages[index - 1];

        current.sender != previous.sender || (current.timestamp - previous.timestamp) > 300000.0
        // 5 minutes gap
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
                        html! {
                            <div class="message-loading">
                                { for (0..3).map(|i| {
                                    html! {
                                        <div key={i} class="message-skeleton" />
                                    }
                                })}
                            </div>
                        }
                    } else {
                        html! {
                            <div class="message-groups">
                                {
                                   {
                                    let mut current_date = String::new();
                                    props.messages.iter().enumerate().map(|(index, msg)| {
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

                                        let show_sender = should_show_sender(&props.messages, index);
                                        let message_class = classes!(
                                            "message-item",
                                            match msg.message_type {
                                                MessageType::System => "system",
                                                MessageType::Text => {
                                                    if msg.sender == props.current_user_id {
                                                        "sent"
                                                    } else {
                                                        "received"
                                                    }
                                                },
                                                MessageType::Error => "error",
                                            }
                                        );

                                        elements.push(html! {
                                            <div class="message-wrapper" key={msg.message_id.clone()}>
                                                if show_sender && msg.message_type != MessageType::System {
                                                    <div class="message-sender">
                                                        { &msg.sender }
                                                    </div>
                                                }
                                                <div class={message_class}>
                                                    <div class="message-content">
                                                        { &msg.content }
                                                    </div>
                                                    <div class="message-meta">
                                                        <span class="message-time">
                                                            { format_time(msg.timestamp) }
                                                        </span>
                                                        if msg.message_type == MessageType::Text {
                                                            <span class={classes!("message-status", msg.status.to_string().to_lowercase())}>
                                                                { get_status_icon(&msg.status) }
                                                            </span>
                                                        }
                                                    </div>
                                                </div>
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
            if *new_messages > 0 && !*auto_scroll {
                <div
                    class="new-messages-indicator"
                    onclick={
                        let on_scroll_to_bottom = props.on_scroll_to_bottom.clone();
                        let new_messages = new_messages.clone();
                        Callback::from(move |e| {
                            new_messages.set(0);
                            on_scroll_to_bottom.emit(e);
                        })
                    }
                >
                    { format!("{} new message{}", *new_messages, if *new_messages == 1 { "" } else { "s" }) }
                </div>
            }
            if props.show_scroll_button {
                <button
                    class="scroll-bottom-button"
                    onclick={props.on_scroll_to_bottom.clone()}
                    title="Scroll to bottom"
                >
                    {"↓"}
                </button>
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
