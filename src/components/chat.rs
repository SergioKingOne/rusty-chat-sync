use crate::components::chat_status::ChatStatus;
use crate::components::message_input::MessageInput;
use crate::components::message_list::MessageList;
use crate::graphql::mutations::{
    CreateMessageResponse, CreateMessageVariables, CREATE_MESSAGE_MUTATION,
};
use crate::graphql::queries::{ListMessagesData, LIST_MESSAGES_QUERY};
use crate::graphql::subscriptions::{OnCreateMessageData, ON_CREATE_MESSAGE_SUBSCRIPTION};
use crate::graphql::types::GraphQLResponse;
use crate::models::message::{Message, MessageStatus};
use crate::state::auth_state::AuthState;
use crate::state::chat_state::{ChatAction, ChatState};
use crate::utils::graphql_client::GraphQLClient;
use crate::utils::websocket::AppSyncWebSocket;
use std::rc::Rc;
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

    // Add this new state for WebSocket
    let ws = use_state(|| None::<Rc<AppSyncWebSocket>>);

    // Initialize chat and subscription
    {
        let chat_state = chat_state.clone();
        let token = props.auth_state.token.clone();
        let ws = ws.clone();

        use_effect_with((), move |_| {
            if let Some(token) = token {
                let token = token.clone();
                let token_for_fetch = token.clone();
                let token_for_sub = token.clone();

                let chat_state_for_fetch = chat_state.clone();

                // Fetch initial messages
                wasm_bindgen_futures::spawn_local(async move {
                    fetch_messages(&chat_state_for_fetch, &token_for_fetch).await;
                });

                // Setup subscription
                let chat_state_for_sub = chat_state.clone();

                let websocket = AppSyncWebSocket::new(
                    "wss://4psoayuvcnfu7ekadjzgs6erli.appsync-realtime-api.us-east-1.amazonaws.com/graphql",
                    &token_for_sub,
                    ON_CREATE_MESSAGE_SUBSCRIPTION,
                    move |payload| {
                        if let Ok(data) = serde_json::from_value::<OnCreateMessageData>(payload) {
                            let new_message = Message::from_message_data(data.message);
                            chat_state_for_sub.dispatch(ChatAction::AddMessage(new_message));
                        }
                    },
                );
                ws.set(Some(Rc::new(websocket)));
            }

            // Cleanup subscription on unmount
            let ws = ws.clone();
            move || {
                if let Some(websocket) = (*ws).clone() {
                    websocket.close();
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
            <MessageList messages={chat_state.messages.clone()} />
            <MessageInput on_send={
                let chat_state = chat_state.clone();
                let token = props.auth_state.token.clone();
                Callback::from(move |msg: Message| {
                    if let Some(token) = token.clone() {
                        handle_message_send(&chat_state, msg, token);
                    } else {
                        chat_state.dispatch(ChatAction::SetError("Not authenticated".to_string()));
                    }
                })
            } />
        </div>
    }
}

async fn fetch_messages(chat_state: &UseReducerHandle<ChatState>, token: &str) {
    chat_state.dispatch(ChatAction::SetLoading(true));

    match GraphQLClient::new().await {
        Ok(client) => {
            let client = client.with_token(token.to_string());
            let result: GraphQLResponse<ListMessagesData> = match client
                .execute_query("ListMessages", LIST_MESSAGES_QUERY, serde_json::json!({}))
                .await
            {
                Ok(result) => result,
                Err(e) => {
                    chat_state.dispatch(ChatAction::SetError(e.to_string()));
                    chat_state.dispatch(ChatAction::SetLoading(false));
                    return;
                }
            };

            if let Some(data) = result.data {
                chat_state.dispatch(ChatAction::SetMessages(
                    data.list_messages
                        .into_iter()
                        .map(Message::from_message_data)
                        .collect(),
                ));
            } else if let Some(errors) = result.errors {
                chat_state.dispatch(ChatAction::SetError(errors[0].message.clone()));
            }
        }
        Err(e) => {
            chat_state.dispatch(ChatAction::SetError(e.to_string()));
        }
    }

    chat_state.dispatch(ChatAction::SetLoading(false));
}

fn handle_message_send(chat_state: &UseReducerHandle<ChatState>, msg: Message, token: String) {
    chat_state.dispatch(ChatAction::AddMessage(msg.clone()));

    // Clone state before moving into async block
    let chat_state = chat_state.clone();

    wasm_bindgen_futures::spawn_local(async move {
        match GraphQLClient::new().await {
            Ok(client) => {
                // Add the token to the client
                let client = client.with_token(token);

                let variables = CreateMessageVariables {
                    content: msg.content.clone(),
                    author: msg.author.clone(),
                };

                match client
                    .execute_query::<_, CreateMessageResponse>(
                        "CreateMessage",
                        CREATE_MESSAGE_MUTATION,
                        variables,
                    )
                    .await
                {
                    Ok(response) => {
                        if let Some(data) = response.data {
                            // Update the message with the server-returned data
                            let server_message = Message::from_message_data(data.create_message);
                            chat_state.dispatch(ChatAction::UpdateMessage(
                                msg.message_id,
                                server_message,
                            ));
                        } else if let Some(errors) = response.errors {
                            chat_state.dispatch(ChatAction::UpdateMessageStatus(
                                msg.message_id,
                                MessageStatus::Failed,
                            ));
                            chat_state.dispatch(ChatAction::SetError(errors[0].message.clone()));
                        }
                    }
                    Err(e) => {
                        chat_state.dispatch(ChatAction::UpdateMessageStatus(
                            msg.message_id,
                            MessageStatus::Failed,
                        ));
                        chat_state.dispatch(ChatAction::SetError(e.to_string()));
                    }
                }
            }
            Err(e) => {
                chat_state.dispatch(ChatAction::UpdateMessageStatus(
                    msg.message_id,
                    MessageStatus::Failed,
                ));
                chat_state.dispatch(ChatAction::SetError(e.to_string()));
            }
        }
    });
}
