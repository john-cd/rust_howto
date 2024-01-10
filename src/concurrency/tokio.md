# Tokio

Tokio is an asynchronous runtime for the Rust programming language. It provides the building blocks needed for writing networking applications.
Tokio provides a few major components:

- Multiple variations of the runtime for executing asynchronous code. Everything from a multi-threaded, work-stealing runtime to a light-weight, single-threaded runtime.
- An asynchronous version of the standard library.
- A large ecosystem of libraries.

## Key links

- [Tokio][tokio]⮳
- [Tokio glossary][tokio-glossary]⮳
- [Tokio tutorial][tokio-tutorial]⮳
- [Tokio examples][tokio-examples]⮳
- [Tokio mini-Redis example][tokio-mini-redis]⮳
- Template for a tokio-rs app with logging & command line argument parser: [rust-tokio-template][rust-tokio-template]⮳

## Graceful shutdown

Example from [tokio_graceful_shutdown][tokio-graceful-shutdown]⮳:

```rust,editable,ignore,noplayground
{{#include ../../deps/examples/tokio_graceful_shutdown.rs}}
```

{{#include ../refs/link-refs.md}}
