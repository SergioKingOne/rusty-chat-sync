use gloo::net::http::Request;
use gloo::storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};

const USER_POOL_ID: &str = "us-east-1_4oNrl079E";
const CLIENT_ID: &str = "p7c55gqav2r7633fgqfbh0rcs";
const AUTH_ENDPOINT: &str = "https://cognito-idp.us-east-1.amazonaws.com";
const STORAGE_KEY: &str = "auth_tokens";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthResponse {
    pub id_token: String,
    pub access_token: String,
}

#[derive(Debug, Serialize)]
struct AuthRequest {
    #[serde(rename = "AuthFlow")]
    auth_flow: String,
    #[serde(rename = "ClientId")]
    client_id: String,
    #[serde(rename = "AuthParameters")]
    auth_parameters: AuthParameters,
}

#[derive(Debug, Serialize)]
struct AuthParameters {
    #[serde(rename = "USERNAME")]
    username: String,
    #[serde(rename = "PASSWORD")]
    password: String,
}

pub struct AuthService;

impl AuthService {
    pub fn new() -> Self {
        Self
    }

    pub async fn login(&self, username: String, password: String) -> Result<AuthResponse, String> {
        let auth_request = AuthRequest {
            auth_flow: "USER_PASSWORD_AUTH".to_string(),
            client_id: CLIENT_ID.to_string(),
            auth_parameters: AuthParameters { username, password },
        };

        let response = Request::post(AUTH_ENDPOINT)
            .header(
                "X-Amz-Target",
                "AWSCognitoIdentityProviderService.InitiateAuth",
            )
            .header("Content-Type", "application/x-amz-json-1.1")
            .json(&auth_request)
            .map_err(|e| e.to_string())?
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if response.ok() {
            let auth_response: AuthResponse = response
                .json()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))?;

            // Store tokens in localStorage
            LocalStorage::set(STORAGE_KEY, &auth_response)
                .map_err(|e| format!("Failed to store tokens: {}", e))?;

            Ok(auth_response)
        } else {
            let error_text = response
                .text()
                .await
                .map_err(|e| format!("Failed to get error text: {}", e))?;
            Err(format!("Authentication failed: {}", error_text))
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
}
