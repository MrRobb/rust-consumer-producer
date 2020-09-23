# Multiple Consumers <-> Multiple Producers Showcase

Example of a consumer <-> producer system in Rust.

![https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fi.stack.imgur.com%2FCoI4J.png&f=1&nofb=1](https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fi.stack.imgur.com%2FCoI4J.png&f=1&nofb=1)

### Requirements

- Rust
- Docker Compose

### Install

```sh
# Install sqlx (cli tool)
cargo install --version=0.1.0-beta.1 sqlx-cli
```

### Usage

```sh
# Initialize RabbitMQ, Postgres and PgAdmin4
docker-compose up

# Setup database
export DATABASE_URL="postgres://guest:guest@localhost/guestdb"
sqlx db create
sqlx migrate run

# Execute consumer
cargo run --release --bin consumer

# Execute producer
cargo run --release --bin producer -- --n-messages 1000
```

> You can go to localhost:15672 to access the RabbitMQ administrator dashboard.

> You can go to localhost:8080 to access the PgAdmin4 administrator panel.
