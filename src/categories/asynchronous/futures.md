# Futures crate

{{#include futures.incl.md}}

[![futures][futures-badge]][futures]  [(crates.io)][futures-crate]  [![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]

The [`futures`][futures]⮳ crate provides a number of core abstractions for writing asynchronous code.

In most cases, you will use this crate directly only when writing async code intended to work for multiple runtimes. Otherwise, use the utilities provided by the ecosystem of your choice - [Tokio](tokio.md) for example.

## Selecting futures

[![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]

`Select` polls multiple futures and streams simultaneously, executing the branch for the future that finishes first. If multiple futures are ready, one will be pseudo-randomly selected at runtime.

```rust,editable,mdbook-runnable
{{#include ../../../deps/tests/futures.rs}}
```

## Joining futures

[![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]

```rust,editable,mdbook-runnable
{{#include ../../../deps/tests/futures2.rs}}
```

## Map, then, either, flatten

[![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]

The `futures` crate provides an extension trait that provides a variety of convenient adapters.

```rust,editable,mdbook-runnable
{{#include ../../../deps/tests/futures3.rs}}
```

## See also

[![futures-executor][futures-executor-badge]][futures-executor]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
