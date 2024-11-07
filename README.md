# Real-time Chat Application

A real-time chat application built with Rust (Yew) for the frontend and AWS AppSync with DynamoDB for the backend. Infrastructure is managed using Terraform.

## Features

- Real-time data synchronization across clients using AWS AppSync subscriptions.
- User authentication and authorization with AWS Cognito.
- Persistent storage of chat messages in Amazon DynamoDB.
- Frontend developed in Rust using the Yew framework, following functional programming principles.
- Test-driven development with comprehensive tests.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/) (for `wasm-pack` and bundling)
- [Terraform](https://www.terraform.io/downloads.html)
- AWS CLI configured with appropriate permissions.

## Setup

### 1. Deploy Infrastructure

Navigate to the `infrastructure` directory and initialize Terraform.

```bash
cd infrastructure
terraform init
terraform apply
```

This will provision AWS AppSync API, DynamoDB tables, and Cognito User Pools. After deployment, note the outputs such as appsync_graphql_endpoint and cognito_client_id.
