use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "infrastructure/appsync/schema.graphql",
    query_path = "frontend/src/graphql/mutations/create_message.graphql",
    response_derives = "Debug, Serialize, Deserialize"
)]
pub struct CreateMessage;
