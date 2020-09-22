# Multiple Consumers <-> Multiple Producers Showcase

Example of a consumer <-> producer system in Rust.

![https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fi.stack.imgur.com%2FCoI4J.png&f=1&nofb=1](https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fi.stack.imgur.com%2FCoI4J.png&f=1&nofb=1)

### Requirements

- Rust
- Docker

### Usage

```sh
# Initialize RabbitMQ service
docker run -it --rm --name rabbitmq -p 5672:5672 -p 15672:15672 rabbitmq:3-management

# Execute consumer
cargo run --release --bin consumer

# Execute producer
cargo run --release --bin producer -- --n-messages 1000
```

> You can go to localhost:15672 to access the administrator dashboard.
