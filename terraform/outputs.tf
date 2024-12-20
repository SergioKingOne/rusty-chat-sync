output "appsync_api_id" {
  description = "AppSync API ID"
  value       = module.appsync.appsync_graphql_api_id
}

output "appsync_graphql_endpoint" {
  description = "AppSync GraphQL API endpoint"
  value       = module.appsync.appsync_graphql_api_uris["GRAPHQL"]
}

output "appsync_realtime_endpoint" {
  description = "AppSync WebSocket endpoint for real-time subscriptions"
  value       = module.appsync.appsync_graphql_api_uris["REALTIME"]
}

output "cognito_user_pool_id" {
  description = "Cognito User Pool ID"
  value       = aws_cognito_user_pool.main.id
}

output "cognito_client_id" {
  description = "Cognito App Client ID"
  value       = aws_cognito_user_pool_client.client.id
}

output "dynamodb_table_name" {
  description = "DynamoDB table name"
  value       = module.dynamodb.table_name
}

output "frontend_bucket_name" {
  description = "Name of the S3 bucket hosting the frontend"
  value       = aws_s3_bucket.frontend.id
}

output "cloudfront_distribution_id" {
  description = "ID of the CloudFront distribution"
  value       = aws_cloudfront_distribution.frontend.id
}

output "cloudfront_domain_name" {
  description = "Domain name of the CloudFront distribution"
  value       = aws_cloudfront_distribution.frontend.domain_name
}
