use crate::graphql::types::{GraphQLRequest, GraphQLResponse};
use reqwest::Client as ReqwestClient;
use serde::{de::DeserializeOwned, Serialize};

pub struct GraphQLClient {
    http_client: ReqwestClient,
    endpoint: String,
    auth_token: Option<String>,
}

impl GraphQLClient {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let http_client = ReqwestClient::builder().build()?;

        Ok(Self {
            http_client,
            endpoint: String::from(
                "https://4psoayuvcnfu7ekadjzgs6erli.appsync-api.us-east-1.amazonaws.com/graphql",
            ),
            auth_token: None,
        })
    }

    pub fn with_token(mut self, token: String) -> Self {
        self.auth_token = Some(token);
        self
    }

    pub async fn execute_query<V, T>(
        &self,
        operation_name: &str,
        query: &str,
        variables: V,
    ) -> Result<GraphQLResponse<T>, Box<dyn std::error::Error>>
    where
        V: Serialize,
        T: DeserializeOwned,
    {
        let request_body = GraphQLRequest {
            query: query.to_string(),
            variables,
            operation_name: operation_name.to_string(),
        };

        let mut request = self
            .http_client
            .post(&self.endpoint)
            .header("Content-Type", "application/json");

        if let Some(token) = &self.auth_token {
            request = request.header("Authorization", token);
        }

        let response = request.json(&request_body).send().await?;
        let response_text = response.text().await?;
        web_sys::console::log_1(&format!("Response: {:?}", &response_text).into());
        let response_body: GraphQLResponse<T> = serde_json::from_str(&response_text)?;
        Ok(response_body)
    }
}
