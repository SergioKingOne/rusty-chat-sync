resource "aws_cognito_user_pool" "main" {
  name = var.cognito_user_pool_name

  alias_attributes = ["email"]

  admin_create_user_config {
    allow_admin_create_user_only = true
  }

  password_policy {
    minimum_length    = 8
    require_uppercase = true
    require_lowercase = true
    require_numbers   = true
    require_symbols   = true
  }

  schema {
    attribute_data_type = "String"
    name                = "email"
    required            = true
    mutable             = true

    string_attribute_constraints {
      min_length = 1
      max_length = 256
    }
  }
}

# Create Cognito User Pool Client
resource "aws_cognito_user_pool_client" "client" {
  name         = "${var.cognito_user_pool_name}-client"
  user_pool_id = aws_cognito_user_pool.main.id

  generate_secret = false
  explicit_auth_flows = [
    "ALLOW_USER_PASSWORD_AUTH",
    "ALLOW_REFRESH_TOKEN_AUTH"
  ]
}

module "dynamodb" {
  source              = "./dynamodb"
  dynamodb_table_name = var.dynamodb_table_name
}

# AppSync configuration
module "appsync" {
  source  = "terraform-aws-modules/appsync/aws"
  version = "2.5.1"

  name                = var.appsync_api_name
  authentication_type = "AMAZON_COGNITO_USER_POOLS"

  user_pool_config = {
    user_pool_id   = aws_cognito_user_pool.main.id
    aws_region     = var.aws_region
    default_action = "ALLOW"
    app_id_client  = aws_cognito_user_pool_client.client.id
  }

  schema = file("${path.module}/appsync/schema.graphql")

  datasources = {
    "MessagesTable" = {
      type = "AMAZON_DYNAMODB"
      dynamodb_config = {
        table_name = module.dynamodb.messages_table_name
        region     = var.aws_region
      }
      service_role_arn    = aws_iam_role.dynamodb_role.arn
      region              = var.aws_region
      table_name          = module.dynamodb.messages_table_name
      create_service_role = false # Since we're using our custom role
    }
  }

  resolvers = {
    "Query.listMessages" = {
      data_source       = "MessagesTable"
      request_template  = file("${path.module}/appsync/resolvers/Query.listMessages.req.vtl")
      response_template = file("${path.module}/appsync/resolvers/Query.listMessages.res.vtl")
    }
    "Mutation.createMessage" = {
      data_source       = "MessagesTable"
      request_template  = file("${path.module}/appsync/resolvers/Mutation.createMessage.req.vtl")
      response_template = file("${path.module}/appsync/resolvers/Mutation.createMessage.res.vtl")
    }
    "Subscription.onCreateMessage" = {
      data_source       = "MessagesTable"
      request_template  = file("${path.module}/appsync/resolvers/Subscription.onCreateMessage.req.vtl")
      response_template = file("${path.module}/appsync/resolvers/Subscription.onCreateMessage.res.vtl")
    }
  }
}

resource "aws_iam_role" "dynamodb_role" {
  name = "appsync-dynamodb-role"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action = "sts:AssumeRole"
        Effect = "Allow"
        Principal = {
          Service = "appsync.amazonaws.com"
        }
      }
    ]
  })
}

resource "aws_iam_role_policy" "dynamodb_policy" {
  name = "appsync-dynamodb-policy"
  role = aws_iam_role.dynamodb_role.id

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect = "Allow"
        Action = [
          "dynamodb:GetItem",
          "dynamodb:PutItem",
          "dynamodb:DeleteItem",
          "dynamodb:UpdateItem",
          "dynamodb:Query",
          "dynamodb:Scan"
        ]
        Resource = [
          module.dynamodb.messages_table_arn,
          "${module.dynamodb.messages_table_arn}/*"
        ]
      }
    ]
  })
}
