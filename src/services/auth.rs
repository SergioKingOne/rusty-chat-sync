use gloo::console::log;
use gloo::net::http::Request;
use gloo::storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

use crate::graphql::mutations::{CreateUserResponse, CreateUserVariables, CREATE_USER_MUTATION};
use crate::utils::config::CONFIG;
use crate::utils::graphql_client::GraphQLClient;

const STORAGE_KEY: &str = "auth_tokens";
const CONTENT_TYPE: &str = "application/x-amz-json-1.1";
const AUTH_FLOW: &str = "USER_PASSWORD_AUTH";
const TARGET_INITIATE_AUTH: &str = "AWSCognitoIdentityProviderService.InitiateAuth";
const TARGET_SIGN_UP: &str = "AWSCognitoIdentityProviderService.SignUp";
const TARGET_CONFIRM_SIGN_UP: &str = "AWSCognitoIdentityProviderService.ConfirmSignUp";
const TARGET_RESEND_CODE: &str = "AWSCognitoIdentityProviderService.ResendConfirmationCode";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthResponse {
    #[serde(rename = "IdToken")]
    pub id_token: String,
    #[serde(rename = "AccessToken")]
    pub access_token: String,
    pub username: String,
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

#[derive(Debug, Serialize)]
struct ResendConfirmationCodeRequest {
    #[serde(rename = "ClientId")]
    client_id: String,
    #[serde(rename = "Username")]
    username: String,
}

pub struct AuthService;

impl AuthService {
    pub fn new() -> Self {
        Self
    }

    pub async fn login(&self, username: String, password: String) -> Result<AuthResponse, String> {
        let username_clone = username.clone();
        let auth_request = AuthRequest {
            auth_flow: AUTH_FLOW.to_string(),
            client_id: CONFIG.cognito_client_id.clone(),
            auth_parameters: AuthParameters { username, password },
        };

        // Convert to string manually to match the exact format
        let request_body = serde_json::to_string(&auth_request)
            .map_err(|e| format!("Failed to serialize request: {}", e))?;

        log!("Login request body:", &request_body);

        let response = Request::post(&CONFIG.cognito_endpoint)
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
                username: username_clone,
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
            client_id: CONFIG.cognito_client_id.to_string(),
            username: username.clone(),
            password,
            user_attributes: vec![UserAttribute {
                name: "email".to_string(),
                value: email.clone(),
            }],
        };

        // Just do the Cognito signup first
        let request_body = serde_json::to_string(&sign_up_request)
            .map_err(|e| format!("Failed to serialize request: {}", e))?;

        let response = Request::post(&CONFIG.cognito_endpoint)
            .header("X-Amz-Target", TARGET_SIGN_UP)
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

        if response.ok() {
            Ok(username)
        } else {
            if response_text.contains("UsernameExistsException") {
                self.resend_confirmation_code(username).await
            } else {
                Err(format!("Sign up failed: {}", response_text))
            }
        }
    }

    async fn resend_confirmation_code(&self, username: String) -> Result<String, String> {
        let resend_request = ResendConfirmationCodeRequest {
            client_id: CONFIG.cognito_client_id.to_string(),
            username: username.clone(),
        };

        let request_body = serde_json::to_string(&resend_request)
            .map_err(|e| format!("Failed to serialize request: {}", e))?;

        log!("Resend code request body:", &request_body);

        let response = Request::post(&CONFIG.cognito_endpoint)
            .header("X-Amz-Target", TARGET_RESEND_CODE)
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

        log!("Resend code response:", &response_text);

        if response.ok() {
            Ok(username)
        } else {
            Err(format!(
                "Failed to resend confirmation code: {}",
                response_text
            ))
        }
    }

    pub fn get_stored_auth() -> Option<AuthResponse> {
        LocalStorage::get::<AuthResponse>(STORAGE_KEY).ok()
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
        password: String,
        email: String,
    ) -> Result<(), String> {
        let confirm_request = ConfirmSignUpRequest {
            client_id: CONFIG.cognito_client_id.to_string(),
            username: username.clone(),
            confirmation_code,
        };

        let request_body = serde_json::to_string(&confirm_request)
            .map_err(|e| format!("Failed to serialize request: {}", e))?;

        log!("Confirm signup request body:", &request_body);

        let response = Request::post(&CONFIG.cognito_endpoint)
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
            // After successful confirmation, login to get tokens
            match self.login(username.clone(), password).await {
                Ok(auth_response) => {
                    // Now create the user in DynamoDB with the token
                    if let Err(e) = self
                        .create_user(&username, &email, &auth_response.id_token)
                        .await
                    {
                        log!("Warning: Failed to create user in DynamoDB: {}", e);
                    }
                    Ok(())
                }
                Err(e) => Err(format!("Failed to login after confirmation: {}", e)),
            }
        } else {
            Err(format!("Failed to confirm signup: {}", response_text))
        }
    }

    async fn create_user(&self, username: &str, email: &str, token: &str) -> Result<(), String> {
        let client = GraphQLClient::new()
            .await
            .map_err(|e| e.to_string())?
            .with_token(token.to_string());

        let variables = CreateUserVariables {
            username: username.to_string(),
            email: email.to_string(),
        };

        let response = client
            .execute_query::<_, CreateUserResponse>("CreateUser", CREATE_USER_MUTATION, variables)
            .await
            .map_err(|e| e.to_string())?;

        if response.errors.is_some() {
            return Err("Failed to create user in database".to_string());
        }

        Ok(())
    }
}
