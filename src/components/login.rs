use crate::services::auth::AuthService;
use crate::state::auth_state::{AuthAction, AuthState};
use gloo::dialogs::alert;
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

    let onsubmit = {
        let username = username.clone();
        let password = password.clone();
        let auth_state = props.auth_state.clone();
        let is_loading = is_loading.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
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
                        alert(&format!("Login failed: {}", e));
                        auth_state.dispatch(AuthAction::SetError(e));
                    }
                }
                is_loading.set(false);
            });
        })
    };

    html! {
        <div class="login-container">
            <h2>{"Login"}</h2>
            <form {onsubmit}>
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
                <button
                    type="submit"
                    disabled={*is_loading}
                >
                    if *is_loading {
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
