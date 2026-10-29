<!-- TODO Reviewed ??-??-2026 -->

# Use Containers with Rust Applications

{{#include using_containers.incl.md}}

Use an image to package a process and its runtime dependencies. Use a Docker client when your application needs to manage containers, and Testcontainers when a test needs an isolated service. Containers share their host's kernel; see [Containerization](containerization.md) for the infrastructure topic.

## Build a Small Application Image

For an ordinary Cargo application with a checked-in `Cargo.lock` and a binary named `my-service`, the following multi-stage Dockerfile uses [cargo-chef][containers~cargo-chef] to cache dependencies. Rename the binary and include application assets as needed. The builder and final image both use Debian 13; native dependencies may require additional runtime libraries.

```dockerfile
FROM rust:1.99.1-slim-trixie AS chef
RUN cargo install cargo-chef --locked --version 0.1.78
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --locked --recipe-path recipe.json
COPY . .
RUN cargo build --release --locked --bin my-service

FROM gcr.io/distroless/cc-debian13:nonroot
COPY --from=builder /app/target/release/my-service /usr/local/bin/my-service
ENTRYPOINT ["/usr/local/bin/my-service"]
```

Build with `docker build -t my-service:local .`, then run `docker run --rm my-service:local`. Exclude `target`, `.git`, local secrets, and other irrelevant files in `.dockerignore`. This template targets a normal application, not this book's collection of independent workspaces and examples.

[cargo-chef][containers~cargo-chef] separates dependency preparation from application compilation so a source-only change can reuse the dependency layer. Keep the Rust toolchain and working directory consistent across cooking and building. Pin deployment images by digest in your release process.

[Distroless][containers~distroless] supplies runtime libraries without a shell or package manager, so use the JSON-array entry point. `scratch` is suitable only when the binary and its runtime requirements are self-contained; a musl target alone does not make arbitrary native dependencies static. See the [cross-compilation guide][containers~cross-compilation] before choosing a static target.

## Manage Containers with Bollard

[![bollard][c~bollard~docs~badge]][c~bollard~docs]

Bollard speaks Docker's API. This recipe lists containers, pulls Alpine, runs a short command in a newly created container, checks its exit code, and removes that container. Docker assigns the name to avoid collisions. If execution and cleanup both fail, both errors are reported. It never stops or removes any container returned by the listing.

```rust,noplayground
{{#include ../../../crates/cats/virtualization/examples/using_containers/docker_client.rs:example}}
```

## Start a Service in a Test

[![testcontainers][c~testcontainers~docs~badge]][c~testcontainers~docs]

Testcontainers starts a service with a dynamically mapped host port. The example waits for Redis's readiness message, discovers that port, sends a RESP `PING`, and verifies `PONG` within a timeout. The container handle controls its lifetime; under the default configuration it is removed when dropped. Setting `TESTCONTAINERS_COMMAND=keep` changes that cleanup behavior for debugging.

```rust,noplayground
{{#include ../../../crates/cats/virtualization/examples/using_containers/redis_test.rs:example}}
```

From `later/crates`, compile the optional examples and run the daemon-dependent tests explicitly:

```sh
cargo test -p virtualization --example using_containers --features containers
cargo test -p virtualization --example using_containers --features containers -- --ignored
```

The first command compiles the tests and reports them as ignored. The second requires a reachable Docker daemon and permission to pull public images and create containers. A missing daemon is an error, not a passing skipped test. Testcontainers uses its documented Docker host resolution; configure the intended daemon explicitly when it differs from the local default.

Mockito mocks HTTP APIs; it does not replace Docker integration or a real database protocol. Use [mocking tools][containers~mocking] for application-unit boundaries and actual services for integration checks.

## Related Topics {#related-topics .skip}

- [Containers](containers.md), [Containerization](containerization.md), and [Virtualization](index.md).
- [Cross-compilation][containers~cross-compilation].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
