use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "terraform/appsync/schema.graphql",
    query_path = "src/graphql/mutations/create_message.graphql",
    response_derives = "Debug, Serialize, Deserialize"
)]
pub struct CreateMessage;
