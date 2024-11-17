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

    // Real-time email validation
    let on_email_change = {
        let email = email.clone();
        let email_error = email_error.clone();

        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            email.set(value.clone());

            if value.is_empty() {
                email_error.set(Some("Email is required".to_string()));
            } else if !value.contains('@') {
                email_error.set(Some("Please enter a valid email address".to_string()));
            } else {
                email_error.set(None);
            }
        })
    };

    // Real-time password validation
    let on_password_change = {
        let password = password.clone();
        let password_error = password_error.clone();
        let confirm_password = confirm_password.clone();
        let confirm_password_error = confirm_password_error.clone();

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

            // Validate confirm password if it's not empty
            if !(*confirm_password).is_empty() && value != *confirm_password {
                confirm_password_error.set(Some("Passwords do not match".to_string()));
            } else {
                confirm_password_error.set(None);
            }
        })
    };

    // Real-time confirm password validation
    let on_confirm_password_change = {
        let confirm_password = confirm_password.clone();
        let confirm_password_error = confirm_password_error.clone();
        let password = password.clone();

        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            confirm_password.set(value.clone());

            if value.is_empty() {
                confirm_password_error.set(Some("Please confirm your password".to_string()));
            } else if value != *password {
                confirm_password_error.set(Some("Passwords do not match".to_string()));
            } else {
                confirm_password_error.set(None);
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

    html! {
        if let Some(username) = (*signed_up_username).clone() {
            <ConfirmSignUp
                username={username}
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
                            onchange={on_password_change}
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
                            onchange={on_confirm_password_change}
                            disabled={*is_loading}
                        />
                        if let Some(error) = (*confirm_password_error).clone() {
                            <div class="error-message">{error}</div>
                        }
                    </div>

                    <button
                        type="submit"
                        class="submit-button"
                        disabled={*is_loading ||
                                username_error.is_some() ||
                                email_error.is_some() ||
                                password_error.is_some() ||
                                confirm_password_error.is_some()}
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
