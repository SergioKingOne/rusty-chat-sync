use aws_config::meta::region::RegionProviderChain;
use aws_sdk_cognitoidentityprovider::Client as CognitoClient;
use graphql_client::Response;
use reqwest::Client as ReqwestClient;
use serde::de::DeserializeOwned;

pub struct GraphQLClient {
    http_client: ReqwestClient,
    endpoint: String,
    cognito_client: CognitoClient,
}

impl GraphQLClient {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
        let config = aws_config::defaults(aws_config::BehaviorVersion::v2024_03_28())
            .region(region_provider)
            .load()
            .await;

        let cognito_client = CognitoClient::new(&config);
        let http_client = ReqwestClient::builder().build()?;

        Ok(Self {
            http_client,
            endpoint: String::from(
                "https://4psoayuvcnfu7ekadjzgs6erli.appsync-api.us-east-1.amazonaws.com/graphql",
            ),
            cognito_client,
        })
    }

    pub async fn execute_query<T: DeserializeOwned>(
        &self,
        operation_name: &str,
        query: &str,
        variables: serde_json::Value,
    ) -> Result<Response<T>, Box<dyn std::error::Error>> {
        let request_body = serde_json::json!({
            "operationName": operation_name,
            "query": query,
            "variables": variables,
        });

        let response = self
            .http_client
            .post(&self.endpoint)
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&request_body)?)
            .send()
            .await?;

        let response_body = response.text().await?;
        let response: Response<T> = serde_json::from_str(&response_body)?;
        Ok(response)
    }

    // TODO: Add method to get Cognito authentication token
    async fn get_auth_token(&self) -> Result<String, Box<dyn std::error::Error>> {
        // Implement Cognito authentication flow
        todo!()
    }
}
