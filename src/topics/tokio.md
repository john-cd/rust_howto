# Tokio

Tokio is an asynchronous runtime for the Rust programming language. It provides the building blocks needed for writing networking applications.

Tokio provides a few major components:

- Multiple variations of the runtime for executing asynchronous code. Everything from a multi-threaded, work-stealing runtime to a light-weight, single-threaded runtime.
- An asynchronous version of the standard library.
- A large ecosystem of libraries.

[Tokio]( https://tokio.rs/ )

[Tokio tutorial]( https://tokio.rs/tokio/tutorial )

[rust-tokio-template]( https://github.com/Finomnis/rust-tokio-template/tree/main ): a template for a tokio-rs app with logging & command line argument parser

[Tokio examples]( https://github.com/tokio-rs/tokio/tree/master/examples )

[Tokio mini-Redis example]( https://github.com/tokio-rs/mini-redis )

## Channels for use in async code

Tokio's `sync` module provides channels for using in async code.

### OneShot

`oneshot` sends a single value from a single producer to a single consumer.
This channel is usually used to send the result of a computation to a waiter.

[Postage](https://lib.rs/crates/postage) is an alternative to `tokio::sync`.

## Alternatives to the Tokio async ecosystem

[async-std]( https://crates.io/crates/async-std ): async version of the Rust standard library. No longer maintained?

[Smol]( https://crates.io/crates/smol )

[Embassy]( https://embassy.dev/ )

[Mio]( https://crates.io/crates/mio ) is a fast, low-level I/O library for Rust focusing on non-blocking APIs and event notification for building high performance I/O apps with as little overhead as possible over the OS abstractions.
