# Futures crate

{{#include futures.incl.md}}

[![futures][c-futures-badge]][c-futures]{{hi:futures}} [![futures-crates.io][c-futures-crates.io-badge]][c-futures-crates.io] [![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]{{hi:Asynchronous}}

The [`futures`][c-futures]{{hi:futures}}⮳ crate provides a number of core abstractions for writing asynchronous code{{hi:Asynchronous}}.

In most cases, you will use this crate directly only when writing async code{{hi:async}} intended to work for multiple runtimes. Otherwise, use the utilities provided by the ecosystem of your choice - [Tokio][p-tokio] for example.

## Selecting futures {#selecting-futures}

[![futures][c-futures-badge]][c-futures]{{hi:futures}} [![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]{{hi:Asynchronous}}

[`futures::future::Select`][c-futures::future::Select]{{hi:futures::future::Select}}⮳ polls multiple futures and streams simultaneously, executing the branch for the future that finishes first. If multiple futures are ready, one will be pseudo-randomly selected at runtime.

```rust,editable
{{#include ../../../deps/tests/categories/asynchronous/futures.rs:example}}
```

## Joining futures {#joining-futures}

[![futures][c-futures-badge]][c-futures]{{hi:futures}} [![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]{{hi:Asynchronous}}

```rust,editable
{{#include ../../../deps/tests/categories/asynchronous/futures2.rs:example}}
```

## Map, then, either, flatten {#map-then-either-flatten}

[![futures][c-futures-badge]][c-futures]{{hi:futures}} [![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]{{hi:Asynchronous}}

The [`futures`][c-futures]{{hi:futures}}⮳ crate provides an extension trait that provides a variety of convenient adapters.

```rust,editable
{{#include ../../../deps/tests/categories/asynchronous/futures3.rs:example}}
```

## See also

[![futures_executor][c-futures_executor-badge]][c-futures_executor]{{hi:futures_executor}}

[p-tokio]: tokio.md
{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO P1 cover future-utils ?

futures = Utility functions for working with Futures and Streams
</div>
