use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "infrastructure/appsync/schema.graphql",
    query_path = "src/graphql/queries/list_messages.graphql",
    response_derives = "Debug, Serialize, Deserialize"
)]
pub struct ListMessages;
