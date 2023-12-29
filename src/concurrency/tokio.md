# Tokio

Tokio is an asynchronous runtime for the Rust programming language. It provides the building blocks needed for writing networking applications.
Tokio provides a few major components:

- Multiple variations of the runtime for executing asynchronous code. Everything from a multi-threaded, work-stealing runtime to a light-weight, single-threaded runtime.
- An asynchronous version of the standard library.
- A large ecosystem of libraries.

## Key links

- [Tokio]( https://tokio.rs/ )
- [Tokio glossary]( https://tokio.rs/tokio/glossary )
- [Tokio tutorial]( https://tokio.rs/tokio/tutorial )
- [Tokio examples]( https://github.com/tokio-rs/tokio/tree/master/examples )
- [Tokio mini-Redis example]( https://github.com/tokio-rs/mini-redis )
- Template for a tokio-rs app with logging & command line argument parser: [rust-tokio-template]( https://github.com/Finomnis/rust-tokio-template/tree/main )

## Graceful shutdown

Example from [tokio_graceful_shutdown]( https://docs.rs/tokio-graceful-shutdown/latest/tokio_graceful_shutdown/ ):

```rust,editable,ignore,mdbook-runnable
{{#include ../../deps/examples/tokio_graceful_shutdown.rs}
```
