// TODO: Not working due to wasm. Not worth fixing for now.

use once_cell::sync::Lazy;
use wasm_bindgen::prelude::*;

// JavaScript interop to get environment variables from window
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn get_env(key: &str) -> Option<String>;
}

#[derive(Debug, Clone)]
pub struct Config {
    pub graphql_endpoint: String,
    pub websocket_endpoint: String,
    pub cognito_client_id: String,
    pub cognito_endpoint: String,
}

// Global config instance that will panic if any required env vars are missing
pub static CONFIG: Lazy<Config> = Lazy::new(|| Config::new().expect("Failed to initialize config"));

impl Config {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            graphql_endpoint: get_required_env("GRAPHQL_ENDPOINT")?,
            websocket_endpoint: get_required_env("WEBSOCKET_ENDPOINT")?,
            cognito_client_id: get_required_env("COGNITO_CLIENT_ID")?,
            cognito_endpoint: get_required_env("COGNITO_ENDPOINT")?,
        })
    }

    pub fn debug_string(&self) -> String {
        format!(
            "Config:\n\
             GraphQL Endpoint: {}\n\
             WebSocket Endpoint: {}\n\
             Cognito Client ID: {}\n\
             Cognito Endpoint: {}\n",
            self.graphql_endpoint,
            self.websocket_endpoint,
            self.cognito_client_id,
            self.cognito_endpoint
        )
    }
}

fn get_required_env(key: &str) -> Result<String, String> {
    get_env(key)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| format!("Missing required environment variable: {}", key))
}

#[cfg(debug_assertions)]
pub fn print_config() {
    web_sys::console::log_1(&format!("{}", CONFIG.debug_string()).into());
}
