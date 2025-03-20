FROM rust:1.85-slim as builder

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates libpq5 &&
    rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /usr/src/app/target/release/actix_poc_scylla .
COPY config.example.toml ./config.toml

EXPOSE 3000
CMD ["./actix_poc_scylla"]
