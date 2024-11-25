resource "aws_cognito_user_pool" "main" {
  name = var.cognito_user_pool_name

  alias_attributes = ["email"]

  auto_verified_attributes = ["email"]

  verification_message_template {
    default_email_option = "CONFIRM_WITH_CODE"
    email_subject        = "Your verification code"
    email_message        = "Your verification code is {####}"
  }

  email_configuration {
    email_sending_account = "COGNITO_DEFAULT"
  }

  admin_create_user_config {
    allow_admin_create_user_only = false
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
    "ALLOW_USER_SRP_AUTH",
    "ALLOW_USER_PASSWORD_AUTH",
    "ALLOW_REFRESH_TOKEN_AUTH"
  ]

  access_token_validity  = 60 # 1 hour
  id_token_validity      = 60 # 1 hour
  refresh_token_validity = 30 # 30 days

  token_validity_units {
    access_token  = "minutes"
    id_token      = "minutes"
    refresh_token = "days"
  }
}

# First, create a local for resolver names to help with dependencies
locals {
  resolver_names = [
    "Query.getUser",
    "Query.getConversation",
    "Query.listUsers",
    "Mutation.createMessage",
    "Mutation.createUser",
    "Mutation.updateUserStatus"
  ]
}

# Create explicit resolver resources
resource "aws_appsync_resolver" "resolvers" {
  for_each = { for name in local.resolver_names : name => {
    type  = split(".", name)[0]
    field = split(".", name)[1]
  } }

  api_id      = module.appsync.appsync_graphql_api_id
  type        = each.value.type
  field       = each.value.field
  data_source = "ChatTable"

  request_template  = file("${path.module}/appsync/resolvers/${each.key}.req.vtl")
  response_template = file("${path.module}/appsync/resolvers/${each.key}.res.vtl")

  depends_on = [module.appsync]
}

# Update the AppSync module to not manage resolvers
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

  # Single data source
  datasources = {
    "ChatTable" = {
      type = "AMAZON_DYNAMODB"
      dynamodb_config = {
        table_name = module.dynamodb.table_name
        region     = var.aws_region
      }
      service_role_arn    = aws_iam_role.dynamodb_role.arn
      region              = var.aws_region
      table_name          = module.dynamodb.table_name
      create_service_role = false
    }
  }

  # Empty resolvers map - we're managing them separately
  resolvers = {}

  depends_on = [module.dynamodb]
}

# Update DynamoDB module configuration
module "dynamodb" {
  source              = "./dynamodb"
  dynamodb_table_name = var.dynamodb_table_name
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
          module.dynamodb.table_arn,
          "${module.dynamodb.table_arn}/*"
        ]
      }
    ]
  })
}

# S3 bucket for frontend hosting
resource "aws_s3_bucket" "frontend" {
  bucket = "${var.project_name}-frontend"
}

resource "aws_s3_bucket_public_access_block" "frontend" {
  bucket = aws_s3_bucket.frontend.id

  block_public_acls       = true
  block_public_policy     = true
  ignore_public_acls      = true
  restrict_public_buckets = true
}

resource "aws_s3_bucket_versioning" "frontend" {
  bucket = aws_s3_bucket.frontend.id
  versioning_configuration {
    status = "Enabled"
  }
}

# CloudFront distribution
resource "aws_cloudfront_distribution" "frontend" {
  enabled             = true
  is_ipv6_enabled     = true
  default_root_object = "index.html"
  price_class         = "PriceClass_100"

  origin {
    domain_name              = aws_s3_bucket.frontend.bucket_regional_domain_name
    origin_access_control_id = aws_cloudfront_origin_access_control.frontend.id
    origin_id                = "S3Origin"
  }

  default_cache_behavior {
    allowed_methods        = ["GET", "HEAD", "OPTIONS"]
    cached_methods         = ["GET", "HEAD"]
    target_origin_id       = "S3Origin"
    viewer_protocol_policy = "redirect-to-https"
    compress               = true

    forwarded_values {
      query_string = false
      cookies {
        forward = "none"
      }
    }

    min_ttl     = 0
    default_ttl = 3600
    max_ttl     = 86400
  }

  custom_error_response {
    error_code         = 404
    response_code      = 200
    response_page_path = "/index.html"
  }

  restrictions {
    geo_restriction {
      restriction_type = "none"
    }
  }

  viewer_certificate {
    cloudfront_default_certificate = true
  }
}

resource "aws_cloudfront_origin_access_control" "frontend" {
  name                              = "${var.project_name}-frontend-oac"
  description                       = "Origin Access Control for Frontend"
  origin_access_control_origin_type = "s3"
  signing_behavior                  = "always"
  signing_protocol                  = "sigv4"
}

# S3 bucket policy
resource "aws_s3_bucket_policy" "frontend" {
  bucket = aws_s3_bucket.frontend.id
  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Sid    = "AllowCloudFrontServicePrincipal"
        Effect = "Allow"
        Principal = {
          Service = "cloudfront.amazonaws.com"
        }
        Action   = "s3:GetObject"
        Resource = "${aws_s3_bucket.frontend.arn}/*"
        Condition = {
          StringEquals = {
            "AWS:SourceArn" = aws_cloudfront_distribution.frontend.arn
          }
        }
      }
    ]
  })
}
