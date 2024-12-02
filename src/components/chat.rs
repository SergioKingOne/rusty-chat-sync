use crate::components::chat_status::ChatStatus;
use crate::components::conversation_list::ConversationList;
use crate::components::message_input::MessageInput;
use crate::components::message_list::MessageList;
use crate::graphql::mutations::{
    CreateMessageResponse, CreateMessageVariables, UpdateUserStatusVariables,
    CREATE_MESSAGE_MUTATION, UPDATE_USER_STATUS_MUTATION,
};
use crate::graphql::queries::{
    GetConversationResponse, ListUsersResponse, GET_CONVERSATION_QUERY, LIST_USERS_QUERY,
};
use crate::graphql::subscriptions::{SubscriptionPayload, ON_CREATE_MESSAGE_SUBSCRIPTION};
use crate::models::message::{Message, MessageStatus};
use crate::state::auth_state::{AuthAction, AuthState};
use crate::state::chat_state::{ChatAction, ChatState};
use crate::utils::graphql_client::GraphQLClient;
use crate::utils::websocket::AppSyncWebSocket;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ChatProps {
    pub auth_state: UseReducerHandle<AuthState>,
    pub on_logout: Callback<()>,
    #[prop_or_default]
    pub selected_user: Option<String>,
    pub on_select_user: Callback<Option<String>>,
}

#[function_component(Chat)]
pub fn chat(props: &ChatProps) -> Html {
    let chat_state = use_reducer(|| ChatState {
        messages: Vec::new(),
        conversations: Vec::new(),
        is_loading: false,
        error: None,
        current_chat_id: None,
        users: Vec::new(),
    });

    let ws = use_state(|| None::<Rc<AppSyncWebSocket>>);
    let show_scroll_bottom = use_state(|| false);
    let show_mobile = use_state(|| false);

    // WebSocket effect
    {
        let chat_state = chat_state.clone();
        let ws = ws.clone();
        let token = props.auth_state.token.clone();
        let chat_id = chat_state.current_chat_id.clone();

        use_effect_with((token, chat_id), move |deps| {
            let (token, chat_id) = deps.clone();
            if let (Some(token), Some(chat_id)) = (token, chat_id) {
                let chat_state = chat_state.clone();
                let websocket = AppSyncWebSocket::new(
                    &token,
                    &ON_CREATE_MESSAGE_SUBSCRIPTION,
                    Some(serde_json::json!({
                        "chatId": chat_id
                    })),
                    move |payload| {
                        if let Ok(subscription_data) =
                            serde_json::from_value::<SubscriptionPayload>(payload)
                        {
                            let message = Message::from_message_data(
                                subscription_data.data.on_create_message,
                            );
                            chat_state.dispatch(ChatAction::AddMessage(message));
                        }
                    },
                );
                ws.set(Some(Rc::new(websocket)));
            }
            || ()
        });
    }

    // Fetch messages effect
    {
        let chat_state = chat_state.clone();
        let auth_state = props.auth_state.clone();
        let token = props.auth_state.token.clone();
        let selected_user = props.selected_user.clone();

        use_effect_with((token, selected_user), move |deps| {
            let (token, selected_user) = deps.clone();
            if let (Some(token), Some(username)) = (token, selected_user) {
                let chat_state = chat_state.clone();
                let auth_state = auth_state.clone();

                let token_clone = token.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    if let Err(e) =
                        fetch_conversation_messages(&chat_state, username.to_string(), &token_clone)
                            .await
                    {
                        if e.contains("expired") || e.contains("token") {
                            auth_state.dispatch(AuthAction::Logout);
                        } else {
                            chat_state.dispatch(ChatAction::SetError(e));
                        }
                    }
                });
            }
            || ()
        });
    }

    // Add conversation selection handler
    let on_select_conversation = {
        let chat_state = chat_state.clone();
        let auth_state = props.auth_state.clone();
        let on_select_user = props.on_select_user.clone();
        let show_mobile = show_mobile.clone();

        Callback::from(move |username: String| {
            let mut users = vec![
                auth_state.user_id.clone().unwrap_or_default(),
                username.clone(),
            ];
            users.sort();
            let chat_id = format!("CHAT#{}#{}", users[0], users[1]);
            chat_state.dispatch(ChatAction::SetCurrentChatId(Some(chat_id)));
            on_select_user.emit(Some(username));
            show_mobile.set(false);
        })
    };

    // Add search handler (minimal for now)
    let on_search_users = Callback::from(|_query: String| {
        // We'll implement this later
    });

    // Update message send handler to match MessageInput's expected type
    let on_send = {
        let chat_state = chat_state.clone();
        let auth_state = props.auth_state.clone();
        let selected_user = props.selected_user.clone();
        let current_user = props.auth_state.user_id.clone();
        let token = props.auth_state.token.clone();

        Callback::from(move |msg: Message| {
            if let (Some(token), Some(receiver), Some(sender)) =
                (token.clone(), selected_user.clone(), current_user.clone())
            {
                let chat_state = chat_state.clone();
                let auth_state = auth_state.clone();
                let content = msg.content.clone();

                let msg = Message::new_text(content, sender.clone(), receiver.clone());
                chat_state.dispatch(ChatAction::AddMessage(msg.clone()));

                let receiver = receiver.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    if let Err(e) = handle_message_send(&chat_state, msg, receiver, &token).await {
                        if e.contains("expired") || e.contains("token") {
                            auth_state.dispatch(AuthAction::Logout);
                        } else {
                            chat_state.dispatch(ChatAction::SetError(e));
                        }
                    }
                });
            } else {
                chat_state.dispatch(ChatAction::SetError(
                    "Please select a user to send message to".to_string(),
                ));
            }
        })
    };

    // Scroll handlers
    let on_scroll = {
        let show_scroll_bottom = show_scroll_bottom.clone();
        Callback::from(move |scroll_info: (f64, f64, f64)| {
            let (scroll_top, scroll_height, client_height) = scroll_info;
            show_scroll_bottom.set(scroll_top + client_height < scroll_height - 50.0);
        })
    };

    let scroll_to_bottom = Callback::from(|e: MouseEvent| {
        e.prevent_default();
        if let Some(list) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.query_selector(".message-list").ok())
            .flatten()
        {
            let list: web_sys::HtmlElement = list.dyn_into().unwrap();
            list.scroll_to_with_x_and_y(0.0, list.scroll_height() as f64);
        }
    });

    // Fetch users effect
    {
        let chat_state = chat_state.clone();
        let auth_state = props.auth_state.clone();
        let token = props.auth_state.token.clone();

        use_effect_with(token, move |token| {
            if let Some(token) = token {
                let chat_state = chat_state.clone();
                let auth_state = auth_state.clone();
                let token_clone = token.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    if let Err(e) = fetch_users(&chat_state, &token_clone).await {
                        if e.contains("expired") || e.contains("token") {
                            auth_state.dispatch(AuthAction::Logout);
                        } else {
                            chat_state.dispatch(ChatAction::SetError(e));
                        }
                    }
                });
            }
            || ()
        });
    }

    html! {
        <div class="chat-container">
            <ConversationList
                conversations={chat_state.conversations.clone()}
                selected_chat_id={chat_state.current_chat_id.clone()}
                on_select={on_select_conversation}
                on_search={on_search_users}
                is_loading={chat_state.is_loading}
                current_user_id={props.auth_state.user_id.clone().unwrap_or_default()}
                users={chat_state.users.clone()}
                show_mobile={*show_mobile}
                on_mobile_toggle={
                    let show = show_mobile.clone();
                    Callback::from(move |_| show.set(!*show))
                }
            />
            <div class="chat-main">
                <div class="chat-header">
                    <h1>{ "Rusty Chat Sync" }</h1>
                    if let Some(username) = &props.selected_user {
                        <h2>{ format!("Chat with {}", username) }</h2>
                    }
                    <button
                        onclick={let cb = props.on_logout.clone(); move |_| cb.emit(())}
                        class="logout-button"
                    >
                        {"Logout"}
                    </button>
                    <ChatStatus
                        is_loading={chat_state.is_loading}
                        error={chat_state.error.clone()}
                        on_clear_error={
                            let chat_state = chat_state.clone();
                            Callback::from(move |_| chat_state.dispatch(ChatAction::ClearError))
                        }
                    />
                </div>
                <MessageList
                    messages={chat_state.messages.clone()}
                    current_user_id={props.auth_state.user_id.clone().unwrap_or_default()}
                    is_loading={chat_state.is_loading}
                    on_scroll={on_scroll}
                    show_scroll_button={*show_scroll_bottom}
                    on_scroll_to_bottom={scroll_to_bottom}
                />
                <MessageInput
                    on_send={on_send}
                    disabled={props.selected_user.is_none()}
                />
            </div>
        </div>
    }
}

async fn fetch_conversation_messages(
    chat_state: &UseReducerHandle<ChatState>,
    other_username: String,
    token: &str,
) -> Result<(), String> {
    chat_state.dispatch(ChatAction::SetLoading(true));

    let client = GraphQLClient::new()
        .await
        .map_err(|e| e.to_string())?
        .with_token(token.to_string());

    let variables = serde_json::json!({
        "otherUsername": other_username
    });

    let response = client
        .execute_query::<_, GetConversationResponse>(
            "GetConversation",
            GET_CONVERSATION_QUERY,
            variables,
        )
        .await
        .map_err(|e| e.to_string())?;

    if let Some(data) = response.data {
        let messages: Vec<Message> = data
            .get_conversation
            .into_iter()
            .map(Message::from_message_data)
            .collect();

        if let Some(first_msg) = messages.first() {
            chat_state.dispatch(ChatAction::SetCurrentChatId(Some(
                first_msg.chat_id.clone(),
            )));
        }

        chat_state.dispatch(ChatAction::SetMessages(messages));
    } else if let Some(errors) = response.errors {
        return Err(errors[0].message.clone());
    }

    chat_state.dispatch(ChatAction::SetLoading(false));
    Ok(())
}

async fn handle_message_send(
    chat_state: &UseReducerHandle<ChatState>,
    msg: Message,
    receiver_username: String,
    token: &str,
) -> Result<(), String> {
    let client = GraphQLClient::new()
        .await
        .map_err(|e| e.to_string())?
        .with_token(token.to_string());

    let variables = CreateMessageVariables {
        content: msg.content.clone(),
        receiver_username,
    };

    let response = client
        .execute_query::<_, CreateMessageResponse>(
            "CreateMessage",
            CREATE_MESSAGE_MUTATION,
            variables,
        )
        .await
        .map_err(|e| e.to_string())?;

    if let Some(data) = response.data {
        let server_message = Message::from_message_data(data.create_message);
        chat_state.dispatch(ChatAction::UpdateMessage(msg.message_id, server_message));
        Ok(())
    } else if let Some(errors) = response.errors {
        chat_state.dispatch(ChatAction::UpdateMessageStatus(
            msg.message_id,
            MessageStatus::Failed,
        ));
        Err(errors[0].message.clone())
    } else {
        Err("Unknown error occurred".to_string())
    }
}

async fn _update_user_status(username: &str, status: &str, token: &str) -> Result<(), String> {
    let client = GraphQLClient::new()
        .await
        .map_err(|e| e.to_string())?
        .with_token(token.to_string());

    let variables = UpdateUserStatusVariables {
        username: username.to_string(),
        status: status.to_string(),
    };

    // Execute query and only check for errors
    client
        .execute_query::<_, serde_json::Value>(
            // Using serde_json::Value instead of specific response type
            "UpdateUserStatus",
            UPDATE_USER_STATUS_MUTATION,
            variables,
        )
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

async fn fetch_users(chat_state: &UseReducerHandle<ChatState>, token: &str) -> Result<(), String> {
    let client = GraphQLClient::new()
        .await
        .map_err(|e| e.to_string())?
        .with_token(token.to_string());

    let response = client
        .execute_query::<_, ListUsersResponse>("ListUsers", LIST_USERS_QUERY, serde_json::json!({}))
        .await
        .map_err(|e| e.to_string())?;

    if let Some(data) = response.data {
        chat_state.dispatch(ChatAction::SetUsers(data.list_users));
        Ok(())
    } else if let Some(errors) = response.errors {
        Err(errors[0].message.clone())
    } else {
        Ok(())
    }
}
