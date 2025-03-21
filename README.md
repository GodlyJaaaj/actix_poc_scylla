# Actix Web API with PostgreSQL & Redis

![Rust](https://img.shields.io/badge/Rust-Language-orange)
![Actix](https://img.shields.io/badge/Actix-Web_Framework-blue)
![PostgreSQL](https://img.shields.io/badge/PostgreSQL-Database-blue)
![Redis](https://img.shields.io/badge/Redis-Session_Store-red)
![Docker](https://img.shields.io/badge/Docker-Container-blue)

A REST API built with Actix Web featuring authentication, user management, and organization/team resources.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.75+)
- [Docker](https://docs.docker.com/get-docker/) and [Docker Compose](https://docs.docker.com/compose/install/)
- [PostgreSQL client](https://www.postgresql.org/download/) (for migrations)
- [Diesel CLI](https://diesel.rs/guides/getting-started) (for migrations)

## Installation

1. Clone the repository:

```bash
git clone https://github.com/GodlyJaaaj/actix_poc_scylla.git
cd actix_poc_scylla
```

2. Create configuration file

```bash
cp config.example.toml config.toml
```

3. Edit the configuration in config.toml with your settings.

4. Start the required services:

```bash
docker-compose up -d
```

5. Run database migrations:

```bash
diesel setup
```

6. Build and run the application:

```bash
cargo run
```

## Configuration

The application uses a TOML configuration file. Here's an example configuration:

```toml
[server]
protocol = "http"
base_url = "0.0.0.0"
port = 3000
env = "dev"

[database]
url = "postgres://postgres:example@localhost/diesel_demo"

[redis]
url = "redis://127.0.0.1:6379"

[session]
secret = "your-very-secure-secret-key-here"

[oauth.google]
client_id = ""
client_secret = ""
redirect_url = ""

[smtp]
server = "smtp.example.com"
port = 587
username = "your-username"
password = "your-password"
tls_mode = "opportunistic"
email_from = "noreply@example.com"
frontend_url = "http://localhost:8080"
```
