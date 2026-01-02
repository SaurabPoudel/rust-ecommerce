# Rust E-commerce API

A professional, high-performance RESTful API for an e-commerce platform built with Rust, Axum, and SQLx.

## 🚀 Features

- **User Management**: Registration and login with JWT-based authentication and Bcrypt password hashing.
- **Product Catalog**: List, view, and manage products.
- **Order System**: Robust order placement with ACID compliance using Postgres transactions.
- **Security**: Protected routes requiring Bearer token authentication.
- **Observability**: Structured logging with `tracing`.
- **Database**: PostgreSQL with `sqlx` for compile-time verified queries.
- **Containerization**: Easy setup with Docker Compose.

## 🛠 Tech Stack

- **Framework**: [Axum](https://github.com/tokio-rs/axum)
- **Runtime**: [Tokio](https://tokio.rs/)
- **Database**: [PostgreSQL](https://www.postgresql.org/)
- **ORM/Query Builder**: [SQLx](https://github.com/launchbadge/sqlx)
- **Authentication**: [JSON Web Tokens (JWT)](https://jwt.io/)
- **Password Hashing**: [Bcrypt](https://en.wikipedia.org/wiki/Bcrypt)

## 📁 Project Structure

```text
src/
├── api/            # Route handlers and API logic
│   ├── auth.rs     # Middleware and JWT extraction
│   ├── users.rs    # User-related endpoints
│   ├── products.rs # Product-related endpoints
│   └── orders.rs   # Order-related endpoints
├── db/             # Database access layer
├── domain/         # Domain models and shared structures
├── config.rs       # Configuration management
├── lib.rs          # Library entry point
├── routes.rs       # Central routing configuration
└── main.rs         # Application entry point
```

## 🏁 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/) (latest stable)
- [Docker](https://www.docker.com/) & [Docker Compose](https://docs.docker.com/compose/)
- [SQLx CLI](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli) (`cargo install sqlx-cli`)

### Setup

1. **Clone the repository**
2. **Setup environment variables**
   ```bash
   cp .env.example .env
   ```
3. **Start the database**
   ```bash
   docker-compose up -d
   ```
4. **Run migrations**
   ```bash
   sqlx migrate run
   ```
5. **Run the application**
   ```bash
   cargo run
   ```

## 🔌 API Endpoints

### Public Endpoints

- `GET /health_check`: Check API and Database status.
- `POST /users/register`: Create a new user account.
- `POST /users/login`: Authenticate and receive a JWT.
- `GET /products`: List all available products.
- `GET /products/{id}`: Get details for a specific product.

### Protected Endpoints (Requires `Authorization: Bearer <token>`)

- `GET /users/me`: Get current authenticated user details.
- `POST /products`: Create a new product.
- `GET /orders`: List your order history.
- `GET /orders/{id}`: Get full details of a specific order.
- `POST /orders`: Place a new order.

## 🧪 Testing

The project includes both unit and integration tests.

```bash
cargo test
```

## 📝 License

This project is licensed under the MIT License.
