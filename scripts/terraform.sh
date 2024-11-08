#!/bin/bash
set -a
source .env
set +a

# Convert .env variables to TF_VAR_ format
export TF_VAR_aws_access_key_id=$AWS_ACCESS_KEY_ID
export TF_VAR_aws_secret_access_key=$AWS_SECRET_ACCESS_KEY
export TF_VAR_aws_region=$AWS_REGION

cd terraform

# Run terraform command with all arguments passed to this script
terraform "$@"