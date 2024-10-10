# Tokio

{{#include tokio.incl.md}}

[![tokio][c-tokio-badge]][c-tokio]  [![tokio-github][c-tokio-github-badge]][c-tokio-github]  [![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]

{{i:Tokio}} is an {{i:asynchronous runtime}} for the Rust programming language. It provides the building blocks needed for writing {{i:networking applications}}. Tokio provides a few major components:

- Multiple variations of the runtime for executing asynchronous code. Everything from a {{i:multi-threaded}}, {{i:work-stealing runtime}} to a light-weight, {{i:single-threaded runtime}}.
- An {{i:asynchronous version}} of the standard library.
- A large ecosystem of libraries.

## Key links

- [`{{i:Tokio}}`][c-tokio]⮳  [![tokio-github][c-tokio-github-badge]][c-tokio-github]
- [`Tokio` glossary][c-tokio-glossary]⮳
- [`Tokio` tutorial][c-tokio-tutorial]⮳
- [![tokio-examples][c-tokio-examples-badge]][c-tokio-examples]
- Tokio mini-Redis example: [![tokio-mini-redis][c-tokio-mini-redis-badge]][c-tokio-mini-redis]
- Template for a tokio-rs app with logging & command line argument parser: [![rust-tokio-template-github][rust-tokio-template-github-badge]][rust-tokio-template-github]

## Graceful shutdown

[![tokio_graceful_shutdown][c-tokio_graceful_shutdown-badge]][c-tokio_graceful_shutdown]  [![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]

Example from [c-tokio_graceful_shutdown][c-tokio_graceful_shutdown]⮳:

```rust,editable,noplayground,no_run
{{#include ../../../deps/tests/tokio_graceful_shutdown.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
