use std::rc::Rc;
use yew::prelude::*;

use crate::services::auth::AuthService;

#[derive(Debug, Clone, PartialEq)]
pub struct AuthState {
    pub is_authenticated: bool,
    pub token: Option<String>,
    pub user_id: Option<String>,
    pub error: Option<String>,
}

pub enum AuthAction {
    SetAuthenticated(String, String), // token, user_id
    SetError(String),
    ClearError,
    Logout,
}

impl Reducible for AuthState {
    type Action = AuthAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut next_state = (*self).clone();

        match action {
            AuthAction::SetAuthenticated(token, user_id) => {
                next_state.is_authenticated = true;
                next_state.token = Some(token);
                next_state.user_id = Some(user_id);
                next_state.error = None;
            }
            AuthAction::SetError(error) => {
                next_state.error = Some(error);
            }
            AuthAction::ClearError => {
                next_state.error = None;
            }
            AuthAction::Logout => {
                AuthService::logout();
                next_state.is_authenticated = false;
                next_state.token = None;
                next_state.user_id = None;
            }
        }

        Rc::new(next_state)
    }
}
