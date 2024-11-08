use crate::components::message_input::MessageInput;
use crate::components::message_list::MessageList;
use crate::models::message::Message;
use wasm_bindgen_futures;
use yew::prelude::*;

#[function_component(Chat)]
pub fn chat() -> Html {
    let messages = use_state(|| Vec::<Message>::new());

    // Fetch initial messages
    {
        let messages = messages.clone();
        use_effect_with(messages, move |_messages| {
            let future = async move {
                // TODO: Implement fetching messages via GraphQL
                // For example, using fetch API or a GraphQL client
                // Update the `messages` state
            };
            wasm_bindgen_futures::spawn_local(future);

            || ()
        });
    }

    html! {
        <div class="chat-container">
            <div class="chat-header">
                <h1>{ "Real-time Chat" }</h1>
            </div>
            <MessageList messages={(*messages).clone()} />
            <MessageInput on_send={Callback::from(move |msg: Message| {
                // TODO: Implement sending message via GraphQL mutation
            })} />
        </div>
    }
}
