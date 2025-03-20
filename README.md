# Scylla POC Actix & Diesel

![Rust](https://img.shields.io/badge/Rust-Language-orange)

## Installation

To run the project, follow these steps:

```sh
docker compose up -d
diesel setup
diesel migration run
cargo run
```

## Environment Variables

The application uses the following environment variables from the `.env` file:

- `DATABASE_URL`: PostgreSQL connection string
- `REDIS_URL`: Redis connection string for session storage
- `SERVER_HOST`: The host IP the server will bind to
- `SERVER_PORT`: The port the server will listen on
- `SECRET_KEY`: Secret key for session encryption (important for production)
- `BCRYPT_COST`: Cost factor for password hashing (default: 10)
