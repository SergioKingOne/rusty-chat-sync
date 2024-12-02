use crate::models::conversation::Conversation;
use crate::models::user::User;
use chrono::{Local, TimeZone};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ConversationListProps {
    pub conversations: Vec<Conversation>,
    pub selected_chat_id: Option<String>,
    pub on_select: Callback<String>, // Callback with username
    pub on_search: Callback<String>,
    pub is_loading: bool,
    pub current_user_id: String,
    pub users: Vec<User>,
    #[prop_or_default]
    pub show_mobile: bool,
    pub on_mobile_toggle: Callback<()>,
}

#[function_component(ConversationList)]
pub fn conversation_list(props: &ConversationListProps) -> Html {
    let search_query = use_state(|| String::new());
    let show_search = use_state(|| false);

    let filtered_users = {
        let query = (*search_query).clone().to_lowercase();
        props
            .users
            .iter()
            .filter(|user| {
                user.username != props.current_user_id
                    && (query.is_empty()
                        || user.username.to_lowercase().contains(&query)
                        || user.email.to_lowercase().contains(&query))
            })
            .cloned()
            .collect::<Vec<_>>()
    };

    // TODO: Search is not implemented yet.
    let _on_search_input = {
        let search_query = search_query.clone();
        let on_search = props.on_search.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            search_query.set(value.clone());
            on_search.emit(value);
        })
    };

    let format_last_seen = |timestamp: f64| {
        let now = Local::now();
        let last_seen = Local.timestamp_millis_opt(timestamp as i64).unwrap();
        let diff = now.signed_duration_since(last_seen);

        if diff.num_minutes() < 1 {
            "just now".to_string()
        } else if diff.num_minutes() < 60 {
            format!("{}m ago", diff.num_minutes())
        } else if diff.num_hours() < 24 {
            format!("{}h ago", diff.num_hours())
        } else {
            last_seen.format("%d %b, %H:%M").to_string()
        }
    };

    html! {
        <>
            <button
                class="mobile-toggle"
                onclick={let cb = props.on_mobile_toggle.clone(); move |_| cb.emit(())}
            >
                {"☰"}
            </button>

            <div class={classes!(
                "conversation-list",
                props.show_mobile.then_some("show")
            )}>
                <div class="conversation-list-header">
                    <h2>{"Conversations"}</h2>
                    <button
                        class="new-chat-button"
                        onclick={let show = show_search.clone(); move |_| show.set(!*show)}
                    >
                        if *show_search {
                            {"×"}
                        } else {
                            {"+"}
                        }
                    </button>
                </div>

                <div class="conversations">
                    if *show_search {
                        if props.is_loading {
                            <div class="user-loading">
                                { for (0..3).map(|i| {
                                    html! {
                                        <div key={i} class="conversation-skeleton" />
                                    }
                                })}
                            </div>
                        } else if filtered_users.is_empty() {
                            <div class="no-users">
                                {"No users found"}
                            </div>
                        } else {
                            <div class="users-list">
                                { for filtered_users.iter().map(|user| {
                                    let username = user.username.clone();
                                    html! {
                                        <div
                                            key={username.clone()}
                                            class="conversation-item"
                                            onclick={
                                                let username = username.clone();
                                                let on_select = props.on_select.clone();
                                                move |_| on_select.emit(username.clone())
                                            }
                                        >
                                            <div class="conversation-avatar">
                                                {&username[0..1].to_uppercase()}
                                            </div>
                                            <div class="conversation-info">
                                                <div class="conversation-name">
                                                    {&username}
                                                    if let Some(status) = &user.status {
                                                        <span class={classes!("user-status", status.to_lowercase())}>
                                                            {status}
                                                        </span>
                                                    }
                                                </div>
                                                <div class="conversation-preview">
                                                    {&user.email}
                                                </div>
                                            </div>
                                        </div>
                                    }
                                })}
                            </div>
                        }
                    } else {
                        if props.is_loading {
                            <div class="conversation-loading">
                                { for (0..3).map(|i| {
                                    html! {
                                        <div key={i} class="conversation-skeleton" />
                                    }
                                })}
                            </div>
                        } else if props.conversations.is_empty() {
                            <div class="no-conversations">
                                {"No conversations yet"}
                            </div>
                        } else {
                            { for props.conversations.iter().map(|conv| {
                                let is_selected = props.selected_chat_id
                                    .as_ref()
                                    .map_or(false, |id| id == &conv.chat_id);

                                let username = conv.other_user.username.clone();

                                html! {
                                    <div
                                        key={conv.chat_id.clone()}
                                        class={classes!(
                                            "conversation-item",
                                            is_selected.then_some("selected")
                                        )}
                                        onclick={
                                            let username = username.clone();
                                            let on_select = props.on_select.clone();
                                            move |_| on_select.emit(username.clone())
                                        }
                                    >
                                        <div class="conversation-avatar">
                                            {&username[0..1].to_uppercase()}
                                        </div>
                                        <div class="conversation-info">
                                            <div class="conversation-name">
                                                {&username}
                                                if let Some(status) = &conv.other_user.status {
                                                    <span class={classes!("user-status", status.to_lowercase())}>
                                                        {status}
                                                    </span>
                                                }
                                            </div>
                                            if let Some(last_message) = &conv.last_message {
                                                <div class="conversation-preview">
                                                    <span class="preview-sender">
                                                        {
                                                            if last_message.sender == props.current_user_id {
                                                                "You: "
                                                            } else {
                                                                ""
                                                            }
                                                        }
                                                    </span>
                                                    {&last_message.content}
                                                </div>
                                                <div class="conversation-time">
                                                    {format_last_seen(last_message.timestamp)}
                                                </div>
                                            }
                                            if conv.unread_count > 0 {
                                                <div class="unread-badge">
                                                    {conv.unread_count}
                                                </div>
                                            }
                                        </div>
                                    </div>
                                }
                            })}
                        }
                    }
                </div>
            </div>
        </>
    }
}
