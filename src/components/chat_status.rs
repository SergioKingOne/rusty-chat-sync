use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ChatStatusProps {
    pub is_loading: bool,
    pub error: Option<String>,
    pub on_clear_error: Callback<()>,
}

#[function_component(ChatStatus)]
pub fn chat_status(props: &ChatStatusProps) -> Html {
    html! {
        <>
            if props.is_loading {
                <div class="loading-indicator">{"Loading..."}</div>
            }
            if let Some(error) = &props.error {
                <div class="error-banner">
                    {error}
                    <button
                        onclick={let cb = props.on_clear_error.clone(); move |_| cb.emit(())}
                    >
                        {"✕"}
                    </button>
                </div>
            }
        </>
    }
} 