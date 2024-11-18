use crate::services::auth::AuthService;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ConfirmSignUpProps {
    pub username: String,
    pub email: String,
    pub password: String,
    pub on_confirmed: Callback<()>,
    pub on_back: Callback<()>,
    #[prop_or_default]
    pub is_resend: bool,
}

#[function_component(ConfirmSignUp)]
pub fn confirm_signup(props: &ConfirmSignUpProps) -> Html {
    let confirmation_code = use_state(|| String::new());
    let is_loading = use_state(|| false);
    let error = use_state(|| Option::<String>::None);

    let onsubmit = {
        let confirmation_code = confirmation_code.clone();
        let username = props.username.clone();
        let password = props.password.clone();
        let email = props.email.clone();
        let is_loading = is_loading.clone();
        let error = error.clone();
        let on_confirmed = props.on_confirmed.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let code = (*confirmation_code).clone();
            let username = username.clone();
            let password = password.clone();
            let email = email.clone();
            let is_loading = is_loading.clone();
            let error = error.clone();
            let on_confirmed = on_confirmed.clone();

            wasm_bindgen_futures::spawn_local(async move {
                is_loading.set(true);
                error.set(None);

                let auth_service = AuthService::new();
                match auth_service
                    .confirm_sign_up(username, code, password, email)
                    .await
                {
                    Ok(()) => {
                        on_confirmed.emit(());
                    }
                    Err(e) => {
                        error.set(Some(e));
                    }
                }
                is_loading.set(false);
            });
        })
    };

    html! {
        <div class="confirm-signup-container">
            <h2>{"Confirm Your Email"}</h2>
            <p>
                if props.is_resend {
                    {"This account exists but needs confirmation. A new confirmation code has been sent to your email."}
                } else {
                    {"Please enter the confirmation code sent to your email"}
                }
            </p>

            <form {onsubmit}>
                if let Some(err) = (*error).clone() {
                    <div class="error-message">{err}</div>
                }

                <div class="form-group">
                    <label for="confirmation-code">{"Confirmation Code"}</label>
                    <input
                        type="text"
                        id="confirmation-code"
                        value={(*confirmation_code).clone()}
                        onchange={let code = confirmation_code.clone(); move |e: Event| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            code.set(input.value());
                        }}
                        disabled={*is_loading}
                    />
                </div>

                <button
                    type="submit"
                    disabled={*is_loading}
                >
                    if *is_loading {
                        {"Confirming..."}
                    } else {
                        {"Confirm"}
                    }
                </button>

                <button
                    type="button"
                    class="link-button"
                    onclick={let cb = props.on_back.clone(); move |_| cb.emit(())}
                    disabled={*is_loading}
                >
                    {"Back to Sign Up"}
                </button>
            </form>
        </div>
    }
}
