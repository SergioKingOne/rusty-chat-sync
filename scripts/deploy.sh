#!/bin/bash

# Check for --force or -f flag
if [[ "$1" != "--force" && "$1" != "-f" ]]; then
    read -p "Warning: This script will deploy infrastructure changes. Continue? (y/N) " confirm
    if [[ $confirm != [yY] ]]; then
        echo "Deployment cancelled"
        exit 1
    fi
fi

set -a
source .env
set +a

# Convert .env variables to TF_VAR_ format
export TF_VAR_aws_access_key_id=$AWS_ACCESS_KEY_ID
export TF_VAR_aws_secret_access_key=$AWS_SECRET_ACCESS_KEY
export TF_VAR_aws_region=$AWS_REGION

cd terraform

terraform init
terraform apply -auto-approve

echo "Deployment complete and cleanup finished"