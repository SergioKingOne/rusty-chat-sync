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
    let username_error = use_state(|| Option::<String>::None);
    let password_error = use_state(|| Option::<String>::None);

    // Real-time username validation
    let on_username_change = {
        let username = username.clone();
        let username_error = username_error.clone();

        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            username.set(value.clone());

            if value.is_empty() {
                username_error.set(Some("Username is required".to_string()));
            } else if value.len() < 3 {
                username_error.set(Some("Username must be at least 3 characters".to_string()));
            } else {
                username_error.set(None);
            }
        })
    };

    // Real-time password validation
    let on_password_change = {
        let password = password.clone();
        let password_error = password_error.clone();

        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            password.set(value.clone());

            if value.is_empty() {
                password_error.set(Some("Password is required".to_string()));
            } else if value.len() < 8 {
                password_error.set(Some("Password must be at least 8 characters".to_string()));
            } else {
                password_error.set(None);
            }
        })
    };

    let onsubmit = {
        let username = username.clone();
        let password = password.clone();
        let auth_state = props.auth_state.clone();
        let is_loading = is_loading.clone();
        let username_error = username_error.clone();
        let password_error = password_error.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            // Final validation before submit
            if username.is_empty()
                || password.is_empty()
                || username_error.is_some()
                || password_error.is_some()
            {
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

            <form {onsubmit} class="login-form">
                <div class={classes!(
                    "form-group",
                    username_error.is_some().then_some("error")
                )}>
                    <label for="username">{"Username"}</label>
                    <input
                        type="text"
                        id="username"
                        class={classes!(
                            "form-input",
                            username_error.is_some().then_some("error")
                        )}
                        placeholder="Enter your username"
                        value={(*username).clone()}
                        onchange={on_username_change}
                        disabled={*is_loading}
                    />
                </div>
                <div class={classes!(
                    "form-group",
                    password_error.is_some().then_some("error")
                )}>
                    <label for="password">{"Password"}</label>
                    <input
                        type="password"
                        id="password"
                        class={classes!(
                            "form-input",
                            password_error.is_some().then_some("error")
                        )}
                        placeholder="Enter your password"
                        value={(*password).clone()}
                        onchange={on_password_change}
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
