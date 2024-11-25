variable "aws_region" {
  description = "AWS region to deploy resources"
  type        = string
  default     = "us-east-1"
}

variable "appsync_api_name" {
  description = "Name of the AppSync API"
  type        = string
  default     = "RealTimeChatAPI"
}

variable "dynamodb_table_name" {
  description = "Name of the DynamoDB table for messages"
  type        = string
  default     = "ChatMessages"
}

variable "cognito_user_pool_name" {
  description = "Name of the Cognito User Pool"
  type        = string
  default     = "ChatUserPool"
}

variable "project_name" {
  description = "Name of the project, used for resource naming"
  type        = string
  default     = "rusty-chat"
}
