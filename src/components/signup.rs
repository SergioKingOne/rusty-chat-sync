use crate::services::auth::AuthService;
use crate::state::auth_state::{AuthAction, AuthState};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SignUpProps {
    pub auth_state: UseReducerHandle<AuthState>,
    pub on_switch_to_login: Callback<()>,
}

#[function_component(SignUp)]
pub fn sign_up(props: &SignUpProps) -> Html {
    let username = use_state(|| String::new());
    let email = use_state(|| String::new());
    let password = use_state(|| String::new());
    let confirm_password = use_state(|| String::new());
    let is_loading = use_state(|| false);
    let error = use_state(|| Option::<String>::None);

    let onsubmit = {
        let username = username.clone();
        let email = email.clone();
        let password = password.clone();
        let confirm_password = confirm_password.clone();
        let is_loading = is_loading.clone();
        let error = error.clone();
        let auth_state = props.auth_state.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            if *password != *confirm_password {
                error.set(Some("Passwords do not match".to_string()));
                return;
            }

            let username_val = (*username).clone();
            let email_val = (*email).clone();
            let password_val = (*password).clone();
            let is_loading = is_loading.clone();
            let error = error.clone();
            let auth_state = auth_state.clone();

            wasm_bindgen_futures::spawn_local(async move {
                is_loading.set(true);
                error.set(None);

                let auth_service = AuthService::new();
                match auth_service
                    .sign_up(username_val, password_val, email_val)
                    .await
                {
                    Ok(()) => {
                        if let Some(auth_response) = AuthService::get_stored_auth() {
                            auth_state.dispatch(AuthAction::SetAuthenticated(
                                auth_response.id_token,
                                auth_response.access_token,
                            ));
                        }
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
        <div class="signup-container">
            <h2>{"Sign Up"}</h2>
            <form {onsubmit}>
                if let Some(err) = (*error).clone() {
                    <div class="error-message">{err}</div>
                }
                <div class="form-group">
                    <label for="username">{"Username"}</label>
                    <input
                        type="text"
                        id="username"
                        value={(*username).clone()}
                        onchange={let username = username.clone(); move |e: Event| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            username.set(input.value());
                        }}
                        disabled={*is_loading}
                    />
                </div>
                <div class="form-group">
                    <label for="email">{"Email"}</label>
                    <input
                        type="email"
                        id="email"
                        value={(*email).clone()}
                        onchange={let email = email.clone(); move |e: Event| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            email.set(input.value());
                        }}
                        disabled={*is_loading}
                    />
                </div>
                <div class="form-group">
                    <label for="password">{"Password"}</label>
                    <input
                        type="password"
                        id="password"
                        value={(*password).clone()}
                        onchange={let password = password.clone(); move |e: Event| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            password.set(input.value());
                        }}
                        disabled={*is_loading}
                    />
                </div>
                <div class="form-group">
                    <label for="confirm-password">{"Confirm Password"}</label>
                    <input
                        type="password"
                        id="confirm-password"
                        value={(*confirm_password).clone()}
                        onchange={let confirm_password = confirm_password.clone(); move |e: Event| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            confirm_password.set(input.value());
                        }}
                        disabled={*is_loading}
                    />
                </div>
                <button
                    type="submit"
                    disabled={*is_loading}
                >
                    if *is_loading {
                        {"Signing up..."}
                    } else {
                        {"Sign Up"}
                    }
                </button>
                <div class="auth-switch">
                    {"Already have an account? "}
                    <button
                        type="button"
                        class="link-button"
                        onclick={let cb = props.on_switch_to_login.clone(); move |_| cb.emit(())}
                        disabled={*is_loading}
                    >
                        {"Login"}
                    </button>
                </div>
            </form>
        </div>
    }
}
