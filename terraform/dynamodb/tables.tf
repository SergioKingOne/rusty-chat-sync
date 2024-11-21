resource "aws_dynamodb_table" "chat" {
  name         = var.dynamodb_table_name
  billing_mode = "PAY_PER_REQUEST"
  hash_key     = "PK"
  range_key    = "SK"

  attribute {
    name = "PK"
    type = "S"
  }

  attribute {
    name = "SK"
    type = "S"
  }

  attribute {
    name = "GSI1PK"
    type = "S"
  }

  attribute {
    name = "GSI1SK"
    type = "S"
  }

  # GSI for accessing messages by conversation
  global_secondary_index {
    name            = "GSI1"
    hash_key        = "GSI1PK"
    range_key       = "GSI1SK"
    projection_type = "ALL"
  }

  tags = {
    Environment = "dev"
    Name        = var.dynamodb_table_name
  }
}

output "chat_table_name" {
  description = "Name of the DynamoDB chat table"
  value       = aws_dynamodb_table.chat.name
}

output "chat_table_arn" {
  description = "ARN of the DynamoDB chat table"
  value       = aws_dynamodb_table.chat.arn
}
