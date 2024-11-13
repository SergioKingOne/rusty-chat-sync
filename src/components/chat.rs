use crate::components::chat_status::ChatStatus;
use crate::components::message_input::MessageInput;
use crate::components::message_list::MessageList;
use crate::graphql::queries::{ListMessagesData, LIST_MESSAGES_QUERY};
use crate::graphql::types::GraphQLResponse;
use crate::models::message::{Message, MessageStatus};
use crate::state::auth_state::AuthState;
use crate::state::chat_state::{ChatAction, ChatState};
use crate::utils::graphql_client::GraphQLClient;
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

    // Initialize chat
    {
        let chat_state = chat_state.clone();
        let token = props.auth_state.token.clone();
        use_effect_with((), move |_| {
            if let Some(token) = token {
                let chat_state = chat_state.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    fetch_messages(&chat_state, &token).await;
                });
            }
            || ()
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
                Callback::from(move |msg: Message| handle_message_send(&chat_state, msg))
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

fn handle_message_send(chat_state: &UseReducerHandle<ChatState>, msg: Message) {
    chat_state.dispatch(ChatAction::AddMessage(msg.clone()));

    // Clone state before moving into async block
    let chat_state = chat_state.clone();
    wasm_bindgen_futures::spawn_local(async move {
        // TODO: Implement GraphQL mutation for sending message

        // Simulate network delay for now
        gloo::timers::future::TimeoutFuture::new(500).await;
        chat_state.dispatch(ChatAction::UpdateMessageStatus(
            msg.message_id,
            MessageStatus::Sent,
        ));
    });
}
