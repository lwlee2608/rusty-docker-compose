# Rusty Docker Compose

`rusty-docker-compose` is a Rust library for managing Docker Compose, specifically designed for integration testing. It provides convenient control over container lifecycles and logging.

## Features

- Start and stop Docker Compose services with ease
- Log container outputs for inspection
- Designed with integration testing in mind

## Installation

Add `rusty-docker-compose` as a dependency in your `Cargo.toml` file:

```toml
[dependencies]
rusty-docker-compose = "0.1.0"
```

## Usage
Here's a basic example of how to use rusty-docker-compose:

```rust
use rusty_docker_compose::docker_compose::DockerCompose;

let docker_compose = DockerCompose::new(
    "tests/docker-compose.yaml",
    "target/docker_logs",
);

docker_compose.up();
// ... run your tests ...
docker_compose.down();
```

For more detailed examples, please refer to the documentation.