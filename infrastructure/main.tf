module "cognito" {
  source  = "terraform-aws-modules/cognito-user-pool/aws"
  version = "~> 4.0"

  name = var.cognito_user_pool_name

  aliases = ["chatapp"]

  admin_create_user_config = {
    allow_admin_create_user_only = true
  }

  password_policy = {
    minimum_length    = 8
    require_uppercase = true
    require_lowercase = true
    require_numbers   = true
    require_symbols   = true
  }
}

module "dynamodb" {
  source = "./dynamodb"
}

module "appsync" {
  source  = "terraform-aws-modules/appsync/aws"
  version = "~> 5.0"

  name                = var.appsync_api_name
  authentication_type = "AMAZON_COGNITO_USER_POOLS"

  user_pool_config = {
    user_pool_id   = module.cognito.user_pool_id
    aws_region     = var.aws_region
    default_action = "ALLOW"
    app_id_1       = module.cognito.client_id
  }

  schema = file("${path.module}/appsync/schema.graphql")

  data_sources = [
    {
      type = "AMAZON_DYNAMODB"
      name = "MessagesTable"
      dynamodb_config = {
        table_name = module.dynamodb.messages_table_name
        aws_region = var.aws_region
      }
    }
  ]

  resolvers = [
    {
      type              = "Query"
      field             = "listMessages"
      data_source       = "MessagesTable"
      request_template  = file("${path.module}/appsync/resolvers/Query.listMessages.req.vtl")
      response_template = file("${path.module}/appsync/resolvers/Query.listMessages.res.vtl")
    },
    {
      type              = "Mutation"
      field             = "createMessage"
      data_source       = "MessagesTable"
      request_template  = file("${path.module}/appsync/resolvers/Mutation.createMessage.req.vtl")
      response_template = file("${path.module}/appsync/resolvers/Mutation.createMessage.res.vtl")
    },
    {
      type              = "Subscription"
      field             = "onCreateMessage"
      request_template  = file("${path.module}/appsync/resolvers/Subscription.onCreateMessage.req.vtl")
      response_template = file("${path.module}/appsync/resolvers/Subscription.onCreateMessage.res.vtl")
    }
  ]
}
