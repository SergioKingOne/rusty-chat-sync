use crate::services::auth::AuthService;
use crate::state::auth_state::{AuthAction, AuthState};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LoginProps {
    pub auth_state: UseReducerHandle<AuthState>,
    pub on_switch_to_signup: Callback<()>,
}

#[function_component(Login)]
pub fn login(props: &LoginProps) -> Html {
    let username = use_state(|| String::new());
    let password = use_state(|| String::new());
    let is_loading = use_state(|| false);
    let validation_error = use_state(|| Option::<String>::None);

    // Add validation before submission
    let validate_form = {
        let username = username.clone();
        let password = password.clone();
        let validation_error = validation_error.clone();

        move || {
            validation_error.set(None);

            if username.is_empty() {
                validation_error.set(Some("Username is required".to_string()));
                return false;
            }

            if password.is_empty() {
                validation_error.set(Some("Password is required".to_string()));
                return false;
            }

            if password.len() < 8 {
                validation_error.set(Some("Password must be at least 8 characters".to_string()));
                return false;
            }

            true
        }
    };

    let onsubmit = {
        let username = username.clone();
        let password = password.clone();
        let auth_state = props.auth_state.clone();
        let is_loading = is_loading.clone();
        let validate = validate_form.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            if !validate() {
                return;
            }

            let username_val = (*username).clone();
            let password_val = (*password).clone();
            let auth_state = auth_state.clone();
            let is_loading = is_loading.clone();

            wasm_bindgen_futures::spawn_local(async move {
                is_loading.set(true);
                let auth_service = AuthService::new();

                match auth_service.login(username_val.clone(), password_val).await {
                    Ok(response) => {
                        auth_state.dispatch(AuthAction::SetAuthenticated(
                            response.id_token,
                            username_val,
                        ));
                    }
                    Err(e) => {
                        auth_state.dispatch(AuthAction::SetError(e));
                    }
                }
                is_loading.set(false);
            });
        })
    };

    html! {
        <div class="login-container">
            <h2>{"Welcome Back"}</h2>
            <p class="login-subtitle">{"Please enter your credentials to continue"}</p>

            if let Some(error) = &props.auth_state.error {
                <div class="error-message">
                    {error}
                </div>
            }

            if let Some(error) = (*validation_error).clone() {
                <div class="error-message">
                    {error}
                </div>
            }

            <form {onsubmit} class="login-form">
                <div class="form-group">
                    <label for="username">{"Username"}</label>
                    <input
                        type="text"
                        id="username"
                        class="form-input"
                        placeholder="Enter your username"
                        value={(*username).clone()}
                        onchange={let username = username.clone(); move |e: Event| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            username.set(input.value());
                        }}
                        disabled={*is_loading}
                    />
                </div>
                <div class="form-group">
                    <label for="password">{"Password"}</label>
                    <input
                        type="password"
                        id="password"
                        class="form-input"
                        placeholder="Enter your password"
                        value={(*password).clone()}
                        onchange={let password = password.clone(); move |e: Event| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            password.set(input.value());
                        }}
                        disabled={*is_loading}
                    />
                </div>
                <button
                    type="submit"
                    class="submit-button"
                    disabled={*is_loading}
                >
                    if *is_loading {
                        <span class="loading-spinner"></span>
                        {"Logging in..."}
                    } else {
                        {"Login"}
                    }
                </button>
            </form>
            <div class="auth-switch">
                {"Don't have an account? "}
                <button
                    type="button"
                    class="link-button"
                    onclick={let cb = props.on_switch_to_signup.clone(); move |_| cb.emit(())}
                    disabled={*is_loading}
                >
                    {"Sign Up"}
                </button>
            </div>
        </div>
    }
}
