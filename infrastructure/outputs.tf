output "appsync_api_id" {
  description = "AppSync API ID"
  value       = module.appsync.appsync_api_id
}

output "appsync_graphql_endpoint" {
  description = "AppSync GraphQL Endpoint"
  value       = module.appsync.graphql_endpoint
}

output "cognito_user_pool_id" {
  description = "Cognito User Pool ID"
  value       = module.cognito.user_pool_id
}

output "cognito_client_id" {
  description = "Cognito App Client ID"
  value       = module.cognito.app_client_id
}
