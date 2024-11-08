output "appsync_api_id" {
  description = "AppSync API ID"
  value       = module.appsync.appsync_graphql_api_id
}

output "debug_appsync" {
  value = module.appsync
}

output "cognito_user_pool_id" {
  description = "Cognito User Pool ID"
  value       = aws_cognito_user_pool.main.id
}

output "cognito_client_id" {
  description = "Cognito App Client ID"
  value       = aws_cognito_user_pool_client.client.id
}
