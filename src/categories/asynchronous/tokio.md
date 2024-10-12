# Tokio

{{#include tokio.incl.md}}

[![tokio][c-tokio-badge]][c-tokio]  [![tokio-github][c-tokio-github-badge]][c-tokio-github]  [![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]

Tokio{{hi:tokio}} is an asynchronous runtime{{hi:Asynchronous runtime}} for the Rust programming language. It provides the building blocks needed for writing networking applications{{hi:Networking applications}}. Tokio provides a few major components:

- Multiple variations of the runtime for executing asynchronous code. Everything from a multi-threaded{{hi:Multi-threading}}, work-stealing runtime{{hi:Work-stealing runtime}} to a light-weight, single-threaded runtime{{hi:Single-threaded runtime}}.
- An asynchronous version of the standard library.
- A large ecosystem of libraries.

## Key links

- [`tokio`][c-tokio]{{hi:tokio}}⮳  [![tokio-github][c-tokio-github-badge]][c-tokio-github]
- [`tokio` glossary][c-tokio_glossary-website]⮳
- [`tokio` tutorial][c-tokio_tutorial]⮳
- [![tokio_examples][c-tokio_examples-badge]][c-tokio_examples]
- Tokio mini-Redis example: [![tokio-mini-redis][c-tokio_mini_redis-github-badge]][c-tokio_mini_redis-github]
- Template for a tokio-rs app with logging & command line argument parser: [![rust-tokio-template-github][rust-tokio-template-github-badge]][rust-tokio-template-github]

## Graceful shutdown

[![tokio_graceful_shutdown][c-tokio_graceful_shutdown-badge]][c-tokio_graceful_shutdown]  [![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]

Example from [c-tokio_graceful_shutdown][c-tokio_graceful_shutdown]⮳:

```rust,editable,noplayground,no_run
{{#include ../../../deps/tests/tokio_graceful_shutdown.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
<div class="hidden">
TODO:
</div>
