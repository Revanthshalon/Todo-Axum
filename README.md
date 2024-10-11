# api-gateway

## Overview
`api-gateway` is a Rust-based API gateway service that integrates with various microservices and databases. It leverages frameworks such as `axum`, `sqlx`, `tokio`, and `async-graphql` to provide a robust and scalable API solution.

## Prerequisites
- Rust (edition 2021)
- Docker
- Docker Compose

## Dependencies
The project uses the following dependencies:
- `axum`
- `sqlx`
- `tokio`
- `shared`
- `tonic`
- `prost`
- `thiserror`
- `chrono`
- `dotenvy`
- `async-graphql`
- `async-graphql-axum`

## Build and Run

### Using Docker Compose
1. Ensure Docker and Docker Compose are installed.
2. Navigate to the project directory.
3. Run the following command to build and start the services:
    ```sh
    docker-compose up --build
    ```

### Manually
1. Ensure Rust is installed.
2. Navigate to the project directory.
3. Build the project using Cargo:
    ```sh
    cargo build
    ```
4. Run the project:
    ```sh
    cargo run
    ```

## Environment Variables
The following environment variables are used in the project:
- `DATABASE_URL`: URL for the PostgreSQL database.
- `TODO_MANAGEMENT_SERVICE_URL`: URL for the Todo Management Service.

## CI/CD
The project uses GitHub Actions for continuous integration. The workflow is defined in `.github/workflows/CI.yml` and includes:
- Lint & Formatting Checks using `cargo clippy` and `cargo fmt`.

## License
This project is licensed under the MIT License.