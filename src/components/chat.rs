use crate::components::chat_status::ChatStatus;
use crate::components::message_input::MessageInput;
use crate::components::message_list::MessageList;
use crate::graphql::mutations::{
    CreateMessageResponse, CreateMessageVariables, UpdateUserStatusVariables,
    CREATE_MESSAGE_MUTATION, UPDATE_USER_STATUS_MUTATION,
};
use crate::graphql::queries::{ListMessagesData, LIST_MESSAGES_QUERY};
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
}

#[function_component(Chat)]
pub fn chat(props: &ChatProps) -> Html {
    let chat_state = use_reducer(|| ChatState {
        messages: Vec::new(),
        is_loading: false,
        error: None,
    });

    let ws = use_state(|| None::<Rc<AppSyncWebSocket>>);
    let show_scroll_bottom = use_state(|| false);

    // Add scroll handler to MessageList
    let on_scroll = {
        let show_scroll_bottom = show_scroll_bottom.clone();

        Callback::from(move |scroll_info: (f64, f64, f64)| {
            let (scroll_top, scroll_height, client_height) = scroll_info;
            // Show button when not at bottom (with small threshold)
            show_scroll_bottom.set(scroll_top + client_height < scroll_height - 50.0);
        })
    };

    // Add scroll to bottom handler
    let scroll_to_bottom = {
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            if let Some(list) = web_sys::window()
                .and_then(|w| w.document())
                .and_then(|d| d.query_selector(".message-list").ok())
                .flatten()
            {
                let list: web_sys::HtmlElement = list.dyn_into().unwrap();
                list.scroll_to_with_x_and_y(0.0, list.scroll_height() as f64);
            }
        })
    };

    let handle_token_error = {
        let auth_state = props.auth_state.clone();
        let chat_state = chat_state.clone();

        move |error: &str| {
            // Check for token expiration related errors
            if error.contains("expired") || error.contains("token") {
                // Dispatch logout action which will redirect to login
                auth_state.dispatch(AuthAction::Logout);
            } else {
                // Handle other errors normally
                chat_state.dispatch(ChatAction::SetError(error.to_string()));
            }
        }
    };

    // Add WebSocket subscription
    {
        let chat_state = chat_state.clone();
        let ws = ws.clone();
        let token = props.auth_state.token.clone();

        use_effect_with(token, move |token| {
            if let Some(token) = token {
                let chat_state = chat_state.clone();
                let websocket = AppSyncWebSocket::new(
                    "wss://4psoayuvcnfu7ekadjzgs6erli.appsync-realtime-api.us-east-1.amazonaws.com/graphql",
                    &token,
                    ON_CREATE_MESSAGE_SUBSCRIPTION,
                    move |payload| {
                        if let Ok(subscription_data) = serde_json::from_value::<SubscriptionPayload>(payload) {
                            let message = Message::from_message_data(
                                subscription_data.data.on_create_message.into_message_data()
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

    // Modify the fetch_messages call
    {
        let chat_state = chat_state.clone();
        let token = props.auth_state.token.clone();
        let handle_token_error = handle_token_error.clone();

        use_effect_with((), move |_| {
            if let Some(token) = token {
                wasm_bindgen_futures::spawn_local(async move {
                    if let Err(e) = fetch_messages(&chat_state, &token).await {
                        handle_token_error(&e);
                    }
                });
            }
            || ()
        });
    }

    // Modify the message send handler
    let on_send = {
        let chat_state = chat_state.clone();
        let token = props.auth_state.token.clone();
        let handle_token_error = handle_token_error.clone();

        Callback::from(move |msg: Message| {
            if let Some(token) = token.clone() {
                let chat_state = chat_state.clone();
                let handle_token_error = handle_token_error.clone();

                chat_state.dispatch(ChatAction::AddMessage(msg.clone()));

                wasm_bindgen_futures::spawn_local(async move {
                    if let Err(e) = handle_message_send(&chat_state, msg, token).await {
                        handle_token_error(&e);
                    }
                });
            } else {
                chat_state.dispatch(ChatAction::SetError("Not authenticated".to_string()));
            }
        })
    };

    // Add user status update
    {
        let auth_state = props.auth_state.clone();
        let token = props.auth_state.token.clone();
        let handle_token_error = handle_token_error.clone();
        let token_for_cleanup = token.clone();

        use_effect_with((), move |_| {
            if let Some(token) = token {
                if let Some(user_id) = auth_state.user_id.clone() {
                    wasm_bindgen_futures::spawn_local(async move {
                        if let Err(e) = update_user_status(&user_id, "online", &token).await {
                            handle_token_error(&e);
                        }
                    });
                }
            }
            move || {
                if let Some(token) = token_for_cleanup {
                    if let Some(user_id) = auth_state.user_id.clone() {
                        wasm_bindgen_futures::spawn_local(async move {
                            let _ = update_user_status(&user_id, "offline", &token).await;
                        });
                    }
                }
            }
        });
    }

    html! {
        <div class="chat-container">
            <div class="chat-header">
                <h1>{ "Rusty Chat Sync" }</h1>
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
                user_id={props.auth_state.user_id.clone().unwrap_or_default()}
            />
        </div>
    }
}

async fn fetch_messages(
    chat_state: &UseReducerHandle<ChatState>,
    token: &str,
) -> Result<(), String> {
    chat_state.dispatch(ChatAction::SetLoading(true));

    let result = match GraphQLClient::new().await {
        Ok(client) => {
            let client = client.with_token(token.to_string());
            match client
                .execute_query::<_, ListMessagesData>(
                    "ListMessages",
                    LIST_MESSAGES_QUERY,
                    serde_json::json!({}),
                )
                .await
            {
                Ok(result) => result,
                Err(e) => return Err(e.to_string()),
            }
        }
        Err(e) => return Err(e.to_string()),
    };

    if let Some(data) = result.data {
        let mut messages: Vec<Message> = data
            .list_messages
            .into_iter()
            .map(Message::from_message_data)
            .collect();
        messages.sort_by(|a, b| a.timestamp.partial_cmp(&b.timestamp).unwrap());
        chat_state.dispatch(ChatAction::SetMessages(messages));
    } else if let Some(errors) = result.errors {
        return Err(errors[0].message.clone());
    }

    chat_state.dispatch(ChatAction::SetLoading(false));
    Ok(())
}

async fn handle_message_send(
    chat_state: &UseReducerHandle<ChatState>,
    msg: Message,
    token: String,
) -> Result<(), String> {
    let client = GraphQLClient::new()
        .await
        .map_err(|e| e.to_string())?
        .with_token(token);

    let variables = CreateMessageVariables {
        content: msg.content.clone(),
        username: msg.username.clone(),
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

async fn update_user_status(username: &str, status: &str, token: &str) -> Result<(), String> {
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
