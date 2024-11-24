use once_cell::sync::Lazy;

#[derive(Debug, Clone)]
pub struct Config {
    pub graphql_endpoint: String,
    pub websocket_endpoint: String,
    pub cognito_client_id: String,
    pub cognito_endpoint: String,
}

// Global config instance with hardcoded values
pub static CONFIG: Lazy<Config> = Lazy::new(|| Config::new());

impl Config {
    pub fn new() -> Self {
        Self {
            graphql_endpoint: "https://4psoayuvcnfu7ekadjzgs6erli.appsync-api.us-east-1.amazonaws.com/graphql".to_string(),
            websocket_endpoint: "wss://4psoayuvcnfu7ekadjzgs6erli.appsync-realtime-api.us-east-1.amazonaws.com/graphql".to_string(),
            cognito_client_id: "p7c55gqav2r7633fgqfbh0rcs".to_string(),
            cognito_endpoint: "https://cognito-idp.us-east-1.amazonaws.com".to_string(),
        }
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

#[cfg(debug_assertions)]
pub fn print_config() {
    web_sys::console::log_1(&format!("{}", CONFIG.debug_string()).into());
}
