# Futures crate

{{#include futures.incl.md}}

[![futures-website][c-futures-website-badge]][c-futures-website] [![futures][c-futures-badge]][c-futures] [![futures-crates.io][c-futures-crates.io-badge]][c-futures-crates.io] [![futures-github][c-futures-github-badge]][c-futures-github] [![futures-lib.rs][c-futures-lib.rs-badge]][c-futures-lib.rs]{{hi:futures}}{{hi:Async}}{{hi:Future}}{{hi:futures}} [![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]{{hi:Asynchronous}}

The [`futures`][c-futures]{{hi:futures}}⮳ crate provides a number of core abstractions for writing asynchronous code{{hi:Asynchronous}}.

In most cases, you will use this crate directly only when writing async code{{hi:async}} intended to work for multiple runtimes. Otherwise, use the utilities provided by the ecosystem of your choice - [Tokio][p-tokio] for example.

## Selecting futures {#selecting-futures}

[![futures-website][c-futures-website-badge]][c-futures-website] [![futures][c-futures-badge]][c-futures] [![futures-crates.io][c-futures-crates.io-badge]][c-futures-crates.io] [![futures-github][c-futures-github-badge]][c-futures-github] [![futures-lib.rs][c-futures-lib.rs-badge]][c-futures-lib.rs]{{hi:futures}}{{hi:Async}}{{hi:Future}}{{hi:futures}} [![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]{{hi:Asynchronous}}

[`futures::future::Select`][c-futures::future::Select]{{hi:futures::future::Select}}⮳ polls multiple futures and streams simultaneously, executing the branch for the future that finishes first. If multiple futures are ready, one will be pseudo-randomly selected at runtime.

```rust,editable
{{#include ../../../crates/ex/cats/asynchronous/tests/futures.rs:example}}
```

## Joining futures {#joining-futures}

[![futures-website][c-futures-website-badge]][c-futures-website] [![futures][c-futures-badge]][c-futures] [![futures-crates.io][c-futures-crates.io-badge]][c-futures-crates.io] [![futures-github][c-futures-github-badge]][c-futures-github] [![futures-lib.rs][c-futures-lib.rs-badge]][c-futures-lib.rs]{{hi:futures}}{{hi:Async}}{{hi:Future}}{{hi:futures}} [![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]{{hi:Asynchronous}}

```rust,editable
{{#include ../../../crates/ex/cats/asynchronous/tests/futures2.rs:example}}
```

## Map, then, either, flatten {#map-then-either-flatten}

[![futures-website][c-futures-website-badge]][c-futures-website] [![futures][c-futures-badge]][c-futures] [![futures-crates.io][c-futures-crates.io-badge]][c-futures-crates.io] [![futures-github][c-futures-github-badge]][c-futures-github] [![futures-lib.rs][c-futures-lib.rs-badge]][c-futures-lib.rs]{{hi:futures}}{{hi:Async}}{{hi:Future}}{{hi:futures}} [![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]{{hi:Asynchronous}}

The [`futures`][c-futures]{{hi:futures}}⮳ crate provides an extension trait that provides a variety of convenient adapters.

```rust,editable
{{#include ../../../crates/ex/cats/asynchronous/tests/futures3.rs:example}}
```

## See also

[![futures_executor][c-futures_executor-badge]][c-futures_executor]{{hi:futures_executor}}

[p-tokio]: tokio.md
{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[P1 cover future-utils ](https://github.com/john-cd/rust_howto/issues/634)?

futures = Utility functions for working with Futures and Streams
</div>
