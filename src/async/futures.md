# Futures crate

[![futures-badge]][futures]  [crates.io][futures-crate]

The [Futures][futures]⮳ crate provides a number of core abstractions for writing asynchronous code.

In most cases, you will use this crate directly only when writing async code intended to work for multiple runtimes. Otherwise, use the utilities provided by the ecosystem of your choice - [Tokio](tokio.md) for example.

## Selecting futures

`Select` polls multiple futures and streams simultaneously, executing the branch for the future that finishes first. If multiple futures are ready, one will be pseudo-randomly selected at runtime.

```rust,editable,mdbook-runnable
{{#include ../../deps/examples/futures.rs}}
```

## Joining futures

```rust,editable,mdbook-runnable
{{#include ../../deps/examples/futures2.rs}}
```

## Map, then, either, flatten

The `futures` crate provides an extension trait that provides a variety of convenient adapters.

```rust,editable,mdbook-runnable
{{#include ../../deps/examples/futures3.rs}}
```

## See also

[![futures-executor-badge]][futures-executor]

{{#include ../refs/link-refs.md}}
