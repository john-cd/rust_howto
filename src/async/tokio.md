# Tokio

[![tokio-badge]][tokio]  [![tokio-github][tokio-github-badge]][tokio-github]

Tokio is an asynchronous runtime for the Rust programming language. It provides the building blocks needed for writing networking applications. Tokio provides a few major components:

- Multiple variations of the runtime for executing asynchronous code. Everything from a multi-threaded, work-stealing runtime to a light-weight, single-threaded runtime.
- An asynchronous version of the standard library.
- A large ecosystem of libraries.

## Key links

- [`Tokio`][tokio]⮳  [![tokio-github][tokio-github-badge]][tokio-github]
- [`Tokio` glossary][tokio-glossary]⮳
- [`Tokio` tutorial][tokio-tutorial]⮳
- [![tokio-examples][tokio-examples-badge]][tokio-examples]
- Tokio mini-Redis example: [![tokio-mini-redis][tokio-mini-redis-badge]][tokio-mini-redis]
- Template for a tokio-rs app with logging & command line argument parser: [![rust-tokio-template-github][rust-tokio-template-github-badge]][rust-tokio-template-github]

## Graceful shutdown

[![tokio_graceful_shutdown-badge]][tokio_graceful_shutdown]

Example from [tokio_graceful_shutdown][tokio_graceful_shutdown]⮳:

```rust,editable,noplayground,no_run
{{#include ../../deps/examples/tokio_graceful_shutdown.rs}}
```

{{#include ../refs/link-refs.md}}
