use crate::graphql::types::{GraphQLRequest, GraphQLResponse};
use crate::utils::config::CONFIG;
use reqwest::Client as ReqwestClient;
use serde::{de::DeserializeOwned, Serialize};

pub struct GraphQLClient {
    http_client: ReqwestClient,
    endpoint: String,
}

impl GraphQLClient {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let http_client = ReqwestClient::builder().build()?;

        Ok(Self {
            http_client,
            endpoint: CONFIG.graphql_endpoint.clone(),
        })
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

        let response = self
            .http_client
            .post(&self.endpoint)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        let response_body = response.json().await?;
        Ok(response_body)
    }
}
