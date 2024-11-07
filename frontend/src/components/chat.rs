use crate::components::message_input::MessageInput;
use crate::components::message_list::MessageList;
use crate::models::message::Message;
use yew::prelude::*;

#[function_component(Chat)]
pub fn chat() -> Html {
    let messages = use_state(|| Vec::<Message>::new());

    // Fetch initial messages
    {
        let messages = messages.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    // TODO: Implement fetching messages via GraphQL
                    // For example, using fetch API or a GraphQL client
                    // Update the `messages` state
                });
                || ()
            },
            (),
        );
    }

    html! {
        <div>
            <h1>{ "Real-time Chat Application" }</h1>
            <MessageList messages={(*messages).clone()} />
            <MessageInput on_send={Callback::from(move |msg: Message| {
                // TODO: Implement sending message via GraphQL mutation
            })} />
        </div>
    }
}
