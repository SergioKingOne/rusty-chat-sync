#!/bin/bash

# Load environment variables
if [ -f .env ]; then
    export $(cat .env | xargs)
else
    echo "Warning: .env file not found. Proceeding with existing environment variables."
fi

# Export AWS credentials explicitly
export AWS_ACCESS_KEY_ID
export AWS_SECRET_ACCESS_KEY
export AWS_REGION

# Build the Rust WASM application
echo "Building Rust WASM application..."
trunk build --release || {
    echo "Error: trunk build failed"
    exit 1
}

# Get the S3 bucket name and CloudFront distribution ID from Terraform output
BUCKET_NAME=$(cd terraform && terraform output -raw frontend_bucket_name)
DISTRIBUTION_ID=$(cd terraform && terraform output -raw cloudfront_distribution_id)

if [ -z "$BUCKET_NAME" ] || [ -z "$DISTRIBUTION_ID" ]; then
    echo "Error: Could not get bucket name or distribution ID from Terraform"
    exit 1
fi

# Sync the built files to S3
echo "Uploading files to S3..."
aws s3 sync dist/ s3://$BUCKET_NAME/ \
    --delete \
    --cache-control "max-age=31536000,public" \
    --exclude "*.html" \
    --exclude "*.wasm"

# Upload HTML and WASM files with different cache settings
aws s3 sync dist/ s3://$BUCKET_NAME/ \
    --delete \
    --cache-control "no-cache" \
    --include "*.html" \
    --include "*.wasm"

# Invalidate CloudFront cache
echo "Invalidating CloudFront cache..."
aws cloudfront create-invalidation \
    --distribution-id $DISTRIBUTION_ID \
    --paths "/*"

echo "Frontend deployment complete!"
