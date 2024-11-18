use super::confirm_signup::ConfirmSignUp;
use crate::services::auth::AuthService;
use crate::state::auth_state::AuthState;
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

    // Validation states
    let username_error = use_state(|| Option::<String>::None);
    let email_error = use_state(|| Option::<String>::None);
    let password_error = use_state(|| Option::<String>::None);
    let confirm_password_error = use_state(|| Option::<String>::None);

    let signed_up_username = use_state(|| Option::<String>::None);

    // Add a new state for general form errors
    let form_error = use_state(|| Option::<String>::None);

    // Real-time username validation
    let on_username_input = {
        let username = username.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            username.set(input.value());
        })
    };

    // Real-time email validation
    let on_email_input = {
        let email = email.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            email.set(input.value());
        })
    };

    // Real-time password validation
    let on_password_input = {
        let password = password.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            password.set(input.value());
        })
    };

    // Real-time confirm password validation
    let on_confirm_password_input = {
        let confirm_password = confirm_password.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            confirm_password.set(input.value());
        })
    };

    // Add onchange handlers for validation
    let on_username_change = {
        let username_error = username_error.clone();

        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();

            if value.is_empty() {
                username_error.set(Some("Username is required".to_string()));
            } else if value.len() < 3 {
                username_error.set(Some("Username must be at least 3 characters".to_string()));
            } else {
                username_error.set(None);
            }
        })
    };

    let on_email_change = {
        let email_error = email_error.clone();

        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();

            if value.is_empty() {
                email_error.set(Some("Email is required".to_string()));
            } else if !value.contains('@') {
                email_error.set(Some("Invalid email format".to_string()));
            } else {
                email_error.set(None);
            }
        })
    };

    let onsubmit = {
        let username = username.clone();
        let email = email.clone();
        let password = password.clone();
        let is_loading = is_loading.clone();
        let signed_up_username = signed_up_username.clone();
        let form_error = form_error.clone();

        // Check if there are any validation errors
        let has_errors = username_error.is_some()
            || email_error.is_some()
            || password_error.is_some()
            || confirm_password_error.is_some();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            if has_errors {
                return;
            }

            let username_val = (*username).clone();
            let email_val = (*email).clone();
            let password_val = (*password).clone();
            let is_loading = is_loading.clone();
            let signed_up_username = signed_up_username.clone();
            let form_error = form_error.clone();

            wasm_bindgen_futures::spawn_local(async move {
                is_loading.set(true);
                form_error.set(None); // Clear previous errors

                let auth_service = AuthService::new();
                match auth_service
                    .sign_up(username_val.clone(), password_val, email_val)
                    .await
                {
                    Ok(username) => {
                        signed_up_username.set(Some(username));
                    }
                    Err(e) => {
                        form_error.set(Some(e.to_string()));
                    }
                }
                is_loading.set(false);
            });
        })
    };

    let is_form_valid = {
        let username = username.clone();
        let email = email.clone();
        let password = password.clone();
        let confirm_password = confirm_password.clone();

        move || {
            !username.is_empty()
                && !email.is_empty()
                && !password.is_empty()
                && !confirm_password.is_empty()
                && password.as_str() == confirm_password.as_str()
                && email.contains('@')
                && username.len() >= 3
                && password.len() >= 8
        }
    };

    html! {
        if let Some(username) = (*signed_up_username).clone() {
            <ConfirmSignUp
                username={username}
                email={(*email).clone()}
                password={(*password).clone()}
                on_confirmed={
                    let on_switch_to_login = props.on_switch_to_login.clone();
                    Callback::from(move |_| on_switch_to_login.emit(()))
                }
                on_back={
                    let signed_up_username = signed_up_username.clone();
                    Callback::from(move |_| signed_up_username.set(None))
                }
            />
        } else {
            <div class="login-container">
                <h2>{"Create Account"}</h2>
                <p class="login-subtitle">{"Please fill in your details to sign up"}</p>

                // Add error message display
                if let Some(error) = (*form_error).clone() {
                    <div class="error-message form-error">{error}</div>
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
                            placeholder="Choose a username"
                            value={(*username).clone()}
                            oninput={on_username_input}
                            onchange={on_username_change}
                            disabled={*is_loading}
                        />
                        if let Some(error) = (*username_error).clone() {
                            <div class="error-message">{error}</div>
                        }
                    </div>

                    <div class={classes!(
                        "form-group",
                        email_error.is_some().then_some("error")
                    )}>
                        <label for="email">{"Email"}</label>
                        <input
                            type="email"
                            id="email"
                            class={classes!(
                                "form-input",
                                email_error.is_some().then_some("error")
                            )}
                            placeholder="Enter your email"
                            value={(*email).clone()}
                            oninput={on_email_input}
                            onchange={on_email_change}
                            disabled={*is_loading}
                        />
                        if let Some(error) = (*email_error).clone() {
                            <div class="error-message">{error}</div>
                        }
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
                            placeholder="Create a password"
                            value={(*password).clone()}
                            oninput={on_password_input}
                            disabled={*is_loading}
                        />
                        if let Some(error) = (*password_error).clone() {
                            <div class="error-message">{error}</div>
                        }
                    </div>

                    <div class={classes!(
                        "form-group",
                        confirm_password_error.is_some().then_some("error")
                    )}>
                        <label for="confirm-password">{"Confirm Password"}</label>
                        <input
                            type="password"
                            id="confirm-password"
                            class={classes!(
                                "form-input",
                                confirm_password_error.is_some().then_some("error")
                            )}
                            placeholder="Confirm your password"
                            value={(*confirm_password).clone()}
                            oninput={on_confirm_password_input}
                            disabled={*is_loading}
                        />
                        if let Some(error) = (*confirm_password_error).clone() {
                            <div class="error-message">{error}</div>
                        }
                    </div>

                    <button
                        type="submit"
                        class="submit-button"
                        disabled={*is_loading || !is_form_valid()}
                    >
                        if *is_loading {
                            <span class="loading-spinner"></span>
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
}
