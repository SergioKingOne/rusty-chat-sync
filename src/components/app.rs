use crate::components::chat::Chat;
use crate::components::login::Login;
use crate::components::signup::SignUp;
use crate::services::auth::AuthService;
use crate::state::auth_state::{AuthAction, AuthState};
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let auth_state = use_reducer(|| {
        // Check for stored auth on initial load
        if let Some(stored_auth) = AuthService::get_stored_auth() {
            AuthState {
                is_authenticated: true,
                token: Some(stored_auth.id_token),
                user_id: Some(stored_auth.access_token),
                error: None,
            }
        } else {
            AuthState {
                is_authenticated: false,
                token: None,
                user_id: None,
                error: None,
            }
        }
    });

    let show_signup = use_state(|| false);

    html! {
        if !auth_state.is_authenticated {
            if *show_signup {
                <SignUp
                    auth_state={auth_state.clone()}
                    on_switch_to_login={
                        let show_signup = show_signup.clone();
                        Callback::from(move |_| show_signup.set(false))
                    }
                />
            } else {
                <Login
                    auth_state={auth_state.clone()}
                    on_switch_to_signup={
                        let show_signup = show_signup.clone();
                        Callback::from(move |_| show_signup.set(true))
                    }
                />
            }
        } else {
            <Chat
                auth_state={auth_state.clone()}
                on_logout={
                    let auth_state = auth_state.clone();
                    Callback::from(move |_| auth_state.dispatch(AuthAction::Logout))
                }
            />
        }
    }
}
