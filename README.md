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
rusty-docker-compose = "0.3.0"
```

## Usage
Here's a basic example of how to use rusty-docker-compose:

```rust
use rusty_docker_compose::DockerComposeCmd;

let docker_compose_cmd = DockerComposeCmd::new(
    "tests/docker-compose.yaml",
    "target/docker_logs",
);

docker_compose_cmd.up();
// ... run your tests ...
docker_compose_cmd.down();
```

Alternatively, you can use `DockerCompose` to automatically start up and tear down the docker compose when it goes out of scope:
```rust
use rusty_docker_compose::DockerCompose;

let _docker_compose = DockerCompose::new(
    "tests/docker-compose.yaml",
    "target/docker_logs",
);

// ... run your tests ...
```

For more detailed examples, please refer to the documentation.