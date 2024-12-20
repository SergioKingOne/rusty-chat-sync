# Rusty Chat Sync

A real-time chat application built with Rust, WebAssembly, and AWS cloud services. The application provides secure authentication and real-time messaging.

## Features

- 🔐 Secure authentication with AWS Cognito
- 💬 Real-time messaging using AppSync WebSocket subscriptions
- 📱 Responsive design for mobile and desktop
- 🔄 Message synchronization and persistence
- 🌐 Scalable serverless architecture
- 🚀 Global content delivery with CloudFront CDN

## Architecture

```mermaid
graph TB
    subgraph "Frontend"
        A[Rust/WASM Client] --> B[WebSocket Connection]
        A --> C[GraphQL Queries/Mutations]
    end

    subgraph "AWS Cloud"
        B --> D[AppSync Real-time API]
        C --> E[AppSync GraphQL API]

        E --> F[DynamoDB]
        D --> F

        G[Cognito User Pool] --> E

        H[IAM Roles/Policies] --> E
        H --> F
    end

    subgraph "Data Model"
        F --> I[Users]
        F --> J[Messages]
        F --> K[Conversations]
    end

    %% Updated Styles with Better Contrast
    style A fill:#4B0082,stroke:#333,stroke-width:2px, color:#ffffff
    style D fill:#FFA500,stroke:#333,stroke-width:2px, color:#000000
    style E fill:#FFA500,stroke:#333,stroke-width:2px, color:#000000
    style F fill:#228B22,stroke:#333,stroke-width:2px, color:#ffffff
    style G fill:#B22222,stroke:#333,stroke-width:2px, color:#ffffff
```

## Technology Stack

- **Frontend**:

  - Rust + WebAssembly
  - Yew Framework
  - GraphQL Client
  - WebSocket for real-time updates

- **Backend**:
  - AWS AppSync (GraphQL API)
  - AWS Cognito (Authentication)
  - AWS DynamoDB (Database)
  - Terraform (Infrastructure as Code)

## Getting Started

### Prerequisites

- Rust and Cargo installed
- Node.js and npm (for development tools)
- AWS Account
- Terraform CLI

### Local Development

1. Clone the repository:

```bash
git clone https://github.com/yourusername/rusty-chat-sync.git
cd rusty-chat-sync
```

2. Set up environment variables:

```bash
cp .env.example .env
```

Edit `.env` with your AWS credentials and other configuration values:

- `AWS_ACCESS_KEY_ID`: Your AWS access key
- `AWS_SECRET_ACCESS_KEY`: Your AWS secret key
- `AWS_REGION`: Your AWS region (e.g., us-east-1)

3. Install dependencies:

```bash
cargo build
```

4. Deploy AWS infrastructure:

```bash
./scripts/terraform.sh init
./scripts/terraform.sh plan   # Optional, to review changes
./scripts/terraform.sh apply
```

5. Configure the application:
   After terraform deployment, copy the outputs and update them in `src/config.rs`. The application uses hardcoded configuration values instead of environment variables due to WebAssembly and project scope limitations.

6. Run the development server:

```bash
trunk serve
```

### Deployment

The application uses Terraform for infrastructure deployment and a script for frontend deployment. To deploy:

1. Initialize and apply Terraform infrastructure:

```bash
./scripts/terraform.sh init
./scripts/terraform.sh apply
```

2. Deploy the frontend:

```bash
./scripts/deploy_frontend.sh
```

This script will:

- Build the Rust WASM application
- Upload the built files to S3
- Configure appropriate cache settings
- Invalidate CloudFront cache

## Project Structure

```
├── Cargo.toml
├── index.html
├── public
│   ├── aws.svg
│   └── styles
├── scripts
│   └── terraform.sh
├── src
│   ├── components    # Yew UI components
│   ├── graphql       # GraphQL queries and mutations
│   ├── models        # Data models
│   ├── services      # Authentication and API services
│   ├── state         # Application state management
│   ├── tests         # Unit tests
│   └── utils         # Helper utilities
└── terraform
    ├── appsync       # GraphQL schema and resolvers
    ├── dynamodb      # Database configuration
    ├── main.tf
    ├── outputs.tf
    └── variables.tf
```
