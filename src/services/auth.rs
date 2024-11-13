use gloo::console::log;
use gloo::net::http::Request;
use gloo::storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

const CLIENT_ID: &str = "p7c55gqav2r7633fgqfbh0rcs";
const AUTH_ENDPOINT: &str = "https://cognito-idp.us-east-1.amazonaws.com";
const STORAGE_KEY: &str = "auth_tokens";
const CONTENT_TYPE: &str = "application/x-amz-json-1.1";
const AUTH_FLOW: &str = "USER_PASSWORD_AUTH";
const TARGET_INITIATE_AUTH: &str = "AWSCognitoIdentityProviderService.InitiateAuth";
const TARGET_SIGN_UP: &str = "AWSCognitoIdentityProviderService.SignUp";
const TARGET_CONFIRM_SIGN_UP: &str = "AWSCognitoIdentityProviderService.ConfirmSignUp";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthResponse {
    #[serde(rename = "IdToken")]
    pub id_token: String,
    #[serde(rename = "AccessToken")]
    pub access_token: String,
}

#[derive(Debug, Deserialize)]
struct CognitoAuthResponse {
    #[serde(rename = "AuthenticationResult")]
    authentication_result: AuthenticationResult,
}

#[derive(Debug, Deserialize)]
struct AuthenticationResult {
    #[serde(rename = "AccessToken")]
    access_token: String,
    #[serde(rename = "IdToken")]
    id_token: String,
}

#[derive(Debug, Deserialize)]
struct SignUpResponse {
    #[serde(rename = "UserConfirmed")]
    user_confirmed: bool,
    #[serde(rename = "UserSub")]
    user_sub: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct AuthRequest {
    auth_flow: String,
    client_id: String,
    auth_parameters: AuthParameters,
}

#[derive(Debug, Serialize)]
struct AuthParameters {
    #[serde(rename = "USERNAME")]
    username: String,
    #[serde(rename = "PASSWORD")]
    password: String,
}

#[derive(Debug, Serialize)]
struct SignUpRequest {
    #[serde(rename = "ClientId")]
    client_id: String,
    #[serde(rename = "Username")]
    username: String,
    #[serde(rename = "Password")]
    password: String,
    #[serde(rename = "UserAttributes")]
    user_attributes: Vec<UserAttribute>,
}

#[derive(Debug, Serialize)]
struct UserAttribute {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Value")]
    value: String,
}

#[derive(Debug, Serialize)]
struct ConfirmSignUpRequest {
    #[serde(rename = "ClientId")]
    client_id: String,
    #[serde(rename = "Username")]
    username: String,
    #[serde(rename = "ConfirmationCode")]
    confirmation_code: String,
}

pub struct AuthService;

impl AuthService {
    pub fn new() -> Self {
        Self
    }

    pub async fn login(&self, username: String, password: String) -> Result<AuthResponse, String> {
        let auth_request = AuthRequest {
            auth_flow: AUTH_FLOW.to_string(),
            client_id: CLIENT_ID.to_string(),
            auth_parameters: AuthParameters { username, password },
        };

        // Convert to string manually to match the exact format
        let request_body = serde_json::to_string(&auth_request)
            .map_err(|e| format!("Failed to serialize request: {}", e))?;

        log!("Login request body:", &request_body);

        let response = Request::post(AUTH_ENDPOINT)
            .header("X-Amz-Target", TARGET_INITIATE_AUTH)
            .header("Content-Type", CONTENT_TYPE)
            .header("Accept", "*/*")
            .body(request_body)
            .map_err(|e| e.to_string())?
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let response_text = response
            .text()
            .await
            .map_err(|e| format!("Failed to get response text: {}", e))?;

        log!("Login response:", &response_text);

        if response.ok() {
            let cognito_response: CognitoAuthResponse = serde_json::from_str(&response_text)
                .map_err(|e| format!("Failed to parse response: {}", e))?;

            let auth_response = AuthResponse {
                id_token: cognito_response.authentication_result.id_token,
                access_token: cognito_response.authentication_result.access_token,
            };

            // Store tokens in localStorage
            LocalStorage::set(STORAGE_KEY, &auth_response)
                .map_err(|e| format!("Failed to store tokens: {}", e))?;

            Ok(auth_response)
        } else {
            Err(format!("Authentication failed: {}", response_text))
        }
    }

    pub async fn sign_up(
        &self,
        username: String,
        password: String,
        email: String,
    ) -> Result<String, String> {
        let sign_up_request = SignUpRequest {
            client_id: CLIENT_ID.to_string(),
            username: username.clone(),
            password: password.clone(),
            user_attributes: vec![UserAttribute {
                name: "email".to_string(),
                value: email,
            }],
        };

        // Log the exact request we're about to send
        let request_body = serde_json::to_string(&sign_up_request)
            .map_err(|e| format!("Failed to serialize request: {}", e))?;

        log!("Request URL: https://cognito-idp.us-east-1.amazonaws.com/");
        log!("Request body:", &request_body);

        // Simplify the request to match the working curl version
        let response = Request::post(AUTH_ENDPOINT)
            .header("X-Amz-Target", TARGET_SIGN_UP)
            .header("Content-Type", CONTENT_TYPE)
            .header("Accept", "*/*")
            .body(request_body) // Use body() instead of json() to have more control
            .map_err(|e| e.to_string())?
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let response_text = response
            .text()
            .await
            .map_err(|e| format!("Failed to get response text: {}", e))?;

        log!("Response status:", response.status());
        log!("Response headers:", format!("{:?}", response.headers()));
        log!("Signup response:", &response_text);

        if response.ok() {
            let signup_response: SignUpResponse = serde_json::from_str(&response_text)
                .map_err(|e| format!("Failed to parse signup response: {}", e))?;

            log!("User confirmed:", signup_response.user_confirmed);
            log!("User sub:", &signup_response.user_sub);

            // Return the username instead of attempting immediate login
            Ok(username)
        } else {
            Err(format!("Sign up failed: {}", response_text))
        }
    }

    pub fn get_stored_auth() -> Option<AuthResponse> {
        LocalStorage::get(STORAGE_KEY).ok()
    }

    pub fn logout() {
        LocalStorage::delete(STORAGE_KEY);
    }

    pub fn is_authenticated() -> bool {
        LocalStorage::get::<AuthResponse>(STORAGE_KEY).is_ok()
    }

    pub async fn confirm_sign_up(
        &self,
        username: String,
        confirmation_code: String,
    ) -> Result<(), String> {
        let confirm_request = ConfirmSignUpRequest {
            client_id: CLIENT_ID.to_string(),
            username,
            confirmation_code,
        };

        let request_body = serde_json::to_string(&confirm_request)
            .map_err(|e| format!("Failed to serialize request: {}", e))?;

        log!("Confirm signup request body:", &request_body);

        let response = Request::post(AUTH_ENDPOINT)
            .header("X-Amz-Target", TARGET_CONFIRM_SIGN_UP)
            .header("Content-Type", CONTENT_TYPE)
            .header("Accept", "*/*")
            .body(request_body)
            .map_err(|e| e.to_string())?
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let response_text = response
            .text()
            .await
            .map_err(|e| format!("Failed to get response text: {}", e))?;

        log!("Confirm signup response:", &response_text);

        if response.ok() {
            Ok(())
        } else {
            Err(format!("Failed to confirm signup: {}", response_text))
        }
    }
}
