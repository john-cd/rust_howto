# Futures Crate

{{#include futures.incl.md}}

[![futures~website][c~futures~website~badge]][c~futures~website] [![futures][c~futures~docs~badge]][c~futures~docs] [![futures~crates.io][c~futures~crates.io~badge]][c~futures~crates.io] [![futures~github][c~futures~github~badge]][c~futures~github] [![futures~lib.rs][c~futures~lib.rs~badge]][c~futures~lib.rs]{{hi:futures}}{{hi:Async}}{{hi:Future}}{{hi:futures}} [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}}

The [`futures`][c~futures~docs]{{hi:futures}}↗ crate provides a number of core abstractions for writing asynchronous code{{hi:Asynchronous}}.

In most cases, you will use this crate directly only when writing [async][p~async] code{{hi:async}} intended to work for multiple runtimes. Otherwise, use the utilities provided by the ecosystem of your choice - [Tokio][p~tokio] for example.

## Selecting Futures {#selecting-futures}

[![futures~website][c~futures~website~badge]][c~futures~website] [![futures][c~futures~docs~badge]][c~futures~docs] [![futures~crates.io][c~futures~crates.io~badge]][c~futures~crates.io] [![futures~github][c~futures~github~badge]][c~futures~github] [![futures~lib.rs][c~futures~lib.rs~badge]][c~futures~lib.rs]{{hi:futures}}{{hi:Async}}{{hi:Future}}{{hi:futures}} [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}}

[`futures::future::Select`][c~futures::future::Select~docs]{{hi:futures::future::Select}}↗ polls multiple futures and streams simultaneously, executing the branch for the future that finishes first. If multiple futures are ready, one will be pseudo-randomly selected at runtime.

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/futures/futures1.rs:example}}
```

## Joining Futures {#joining-futures}

[![futures~website][c~futures~website~badge]][c~futures~website] [![futures][c~futures~docs~badge]][c~futures~docs] [![futures~crates.io][c~futures~crates.io~badge]][c~futures~crates.io] [![futures~github][c~futures~github~badge]][c~futures~github] [![futures~lib.rs][c~futures~lib.rs~badge]][c~futures~lib.rs]{{hi:futures}}{{hi:Async}}{{hi:Future}}{{hi:futures}} [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}}

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/futures/futures2.rs:example}}
```

## Map, `then`, `either`, `flatten` {#map-then-either-flatten}

[![futures~website][c~futures~website~badge]][c~futures~website] [![futures][c~futures~docs~badge]][c~futures~docs] [![futures~crates.io][c~futures~crates.io~badge]][c~futures~crates.io] [![futures~github][c~futures~github~badge]][c~futures~github] [![futures~lib.rs][c~futures~lib.rs~badge]][c~futures~lib.rs]{{hi:futures}}{{hi:Async}}{{hi:Future}}{{hi:futures}} [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}}

The [`futures`][c~futures~docs]{{hi:futures}}↗ crate provides an extension trait that provides a variety of convenient adapters.

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/futures/futures3.rs:example}}
```

## `futures-util` {#futures-util}

[![future-utils][c~future-utils~docs~badge]][c~future-utils~docs] [![future-utils~crates.io][c~future-utils~crates.io~badge]][c~future-utils~crates.io] [![future-utils~github][c~future-utils~github~badge]][c~future-utils~github] [![future-utils~lib.rs][c~future-utils~lib.rs~badge]][c~future-utils~lib.rs]{{hi:future-utils}}{{hi:Async}}{{hi:Futures}}{{hi:Tokio}} [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}}

Common utilities and extension [traits][p~traits] for the [`futures-rs`][c~futures~docs]↗{{hi:futures-rs}} library. Extensions to Rust's [`Future`][c~std::future::Future~docs]↗{{hi:std::future::Future}} and [`Stream`][c~futures::prelude::Stream~docs]↗{{hi:futures::prelude::Stream}} traits. Combinators and utilities for working with [`Future`][c~futures~docs]↗{{hi:Futures}}, `Stream`, [`Sink`][c~futures::prelude::Sink~docs]↗{{hi:futures::prelude::Sink}}, and the [`AsyncRead`][c~futures::prelude::AsyncRead~docs]:↗ and [`AsyncWrite`][c~futures::prelude::AsyncWrite~docs]↗ traits.

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/futures/futures_util.rs:example}}
```

## Related Topics {#related-topics}

- [[tokio | Tokio]].

## See Also

[![futures-executor][c~futures-executor~docs~badge]][c~futures-executor~docs]{{hi:futures-executor}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[futures = Utility [functions][p~functions] for working with Futures and Streams](https://github.com/john-cd/rust_howto/issues/1340)
</div>
