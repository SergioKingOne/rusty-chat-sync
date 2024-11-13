use crate::components::login::Login;
use crate::components::message_input::MessageInput;
use crate::components::message_list::MessageList;
use crate::components::signup::SignUp;
use crate::graphql::queries::{ListMessagesData, LIST_MESSAGES_QUERY};
use crate::graphql::types::GraphQLResponse;
use crate::models::message::{Message, MessageStatus};
use crate::services::auth::AuthService;
use crate::state::auth_state::{AuthAction, AuthState};
use crate::state::chat_state::{ChatAction, ChatState};
use crate::utils::graphql_client::GraphQLClient;
use yew::prelude::*;

#[function_component(Chat)]
pub fn chat() -> Html {
    let auth_state = use_reducer(|| {
        // Check for stored auth on initial load
        if let Some(stored_auth) = AuthService::get_stored_auth() {
            AuthState {
                is_authenticated: true,
                token: Some(stored_auth.id_token),
                user_id: Some(stored_auth.access_token),
                error: None,
            }
        } else {
            AuthState {
                is_authenticated: false,
                token: None,
                user_id: None,
                error: None,
            }
        }
    });

    let chat_state = use_reducer(|| ChatState {
        messages: Vec::new(),
        is_loading: false,
        error: None,
    });

    let show_signup = use_state(|| false);

    // Initialize chat when authenticated
    {
        let chat_state = chat_state.clone();
        let auth_state = auth_state.clone();
        use_effect_with(auth_state.is_authenticated, move |_| {
            if auth_state.is_authenticated {
                let chat_state = chat_state.clone();
                let token = auth_state.token.clone().unwrap();
                wasm_bindgen_futures::spawn_local(async move {
                    chat_state.dispatch(ChatAction::SetLoading(true));

                    match GraphQLClient::new().await {
                        Ok(client) => {
                            let client = client.with_token(token);
                            let result: GraphQLResponse<ListMessagesData> = match client
                                .execute_query(
                                    "ListMessages",
                                    LIST_MESSAGES_QUERY,
                                    serde_json::json!({}),
                                )
                                .await
                            {
                                Ok(result) => result,
                                Err(e) => {
                                    chat_state.dispatch(ChatAction::SetError(e.to_string()));
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
                                chat_state
                                    .dispatch(ChatAction::SetError(errors[0].message.clone()));
                            }
                        }
                        Err(e) => {
                            chat_state.dispatch(ChatAction::SetError(e.to_string()));
                        }
                    }

                    chat_state.dispatch(ChatAction::SetLoading(false));
                });
            }
            || ()
        });
    }

    let on_send = {
        let chat_state = chat_state.clone();
        Callback::from(move |msg: Message| {
            let chat_state = chat_state.clone();
            chat_state.dispatch(ChatAction::AddMessage(msg.clone()));

            // TODO: Implement actual message sending to backend
            wasm_bindgen_futures::spawn_local(async move {
                // Simulate network delay for now
                gloo::timers::future::TimeoutFuture::new(500).await;
                chat_state.dispatch(ChatAction::UpdateMessageStatus(
                    msg.message_id,
                    MessageStatus::Sent,
                ));
            });
        })
    };

    html! {
        <div class="chat-container">
            if !auth_state.is_authenticated {
                if *show_signup {
                    <SignUp
                        auth_state={auth_state.clone()}
                        on_switch_to_login={
                            let show_signup = show_signup.clone();
                            Callback::from(move |_| show_signup.set(false))
                        }
                    />
                } else {
                    <Login
                        auth_state={auth_state.clone()}
                        on_switch_to_signup={
                            let show_signup = show_signup.clone();
                            Callback::from(move |_| show_signup.set(true))
                        }
                    />
                }
            } else {
                <div class="chat-header">
                    <h1>{ "Rusty Chat Sync" }</h1>
                    <button
                        onclick={
                            let auth_state = auth_state.clone();
                            Callback::from(move |_| {
                                auth_state.dispatch(AuthAction::Logout);
                            })
                        }
                        class="logout-button"
                    >
                        {"Logout"}
                    </button>
                    if chat_state.is_loading {
                        <div class="loading-indicator">{"Loading..."}</div>
                    }
                    if let Some(error) = &chat_state.error {
                        <div class="error-banner">
                            {error}
                            <button
                                onclick={
                                    let chat_state = chat_state.clone();
                                    move |_| chat_state.dispatch(ChatAction::ClearError)
                            }>{"✕"}</button>
                        </div>
                    }
                </div>
                <MessageList
                    messages={chat_state.messages.clone()}
                />
                <MessageInput {on_send} />
            }
        </div>
    }
}
