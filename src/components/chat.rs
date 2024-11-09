use crate::components::message_input::MessageInput;
use crate::components::message_list::MessageList;
use crate::graphql::mutations::CreateMessage;
use crate::graphql::queries::ListMessages;
use crate::models::message::{Message, MessageStatus};
use crate::state::chat_state::{ChatAction, ChatState};
use crate::utils::graphql_client::GraphQLClient;
use yew::prelude::*;

#[function_component(Chat)]
pub fn chat() -> Html {
    let chat_state = use_reducer(|| ChatState {
        messages: Vec::new(),
        is_loading: false,
        error: None,
    });

    // Initialize chat
    {
        let chat_state = chat_state.clone();
        use_effect_with(
            (), // Run once on mount
            move |_| {
                let chat_state = chat_state.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    chat_state.dispatch(ChatAction::SetLoading(true));

                    match GraphQLClient::new().await {
                        Ok(client) => {
                            // Fetch existing messages
                            match client
                                .query::<ListMessages, _>(ListMessages::Variables {})
                                .await
                            {
                                Ok(response) => {
                                    if let Some(messages) = response.data {
                                        chat_state.dispatch(ChatAction::SetMessages(
                                            messages
                                                .list_messages
                                                .into_iter()
                                                .map(|m| Message::from(m))
                                                .collect(),
                                        ));
                                    }
                                }
                                Err(e) => {
                                    chat_state.dispatch(ChatAction::SetError(e.to_string()));
                                }
                            }
                        }
                        Err(e) => {
                            chat_state.dispatch(ChatAction::SetError(e.to_string()));
                        }
                    }

                    chat_state.dispatch(ChatAction::SetLoading(false));
                });
                || ()
            },
        );
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
            <div class="chat-header">
                <h1>{ "Rusty Chat Sync" }</h1>
                if chat_state.is_loading {
                    <div class="loading-indicator">{"Loading..."}</div>
                }
                if let Some(error) = &chat_state.error {
                    <div class="error-banner">
                        {error}
                        <button onclick={
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
        </div>
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use wasm_bindgen_test::*;

//     wasm_bindgen_test_configure!(run_in_browser);

//     #[wasm_bindgen_test]
//     async fn test_message_flow() {
//         use yew::platform::spawn_local;

//         // Create test handle for the component
//         let handle = yew::Renderer::<Chat>::new().render();

//         // Create message and send it to component
//         let msg = Message::new_text("Hello".to_string(), "User".to_string());
//         handle.send_message(ChatAction::AddMessage(msg.clone()));

//     }
// }
