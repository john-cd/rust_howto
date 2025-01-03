# Actors in Rust

{{#include _actors.incl.md}}

## `stakker` {#stakker}

[![stakker-website][c-stakker-website-badge]][c-stakker-website] [![stakker][c-stakker-badge]][c-stakker] [![stakker-crates.io][c-stakker-crates.io-badge]][c-stakker-crates.io] [![stakker-github][c-stakker-github-badge]][c-stakker-github] [![stakker-lib.rs][c-stakker-lib.rs-badge]][c-stakker-lib.rs]{{hi:stakker}}{{hi:Runtime}}{{hi:Erlang}}{{hi:Async}}{{hi:Actor}}{{hi:Pony}} [![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]{{hi:Asynchronous}} [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]{{hi:Concurrency}} [![cat-data-structures][cat-data-structures-badge]][cat-data-structures]{{hi:Data structures}}

A lightweight low-level single-threaded actor runtime.

```rust,editable
{{#include ../../../crates/ex/categories/c/tests/concurrency/stakker.rs:example}}
```

## `riker` {#riker}

[![riker-website][c-riker-website-badge]][c-riker-website] [![riker][c-riker-badge]][c-riker] [![riker-crates.io][c-riker-crates.io-badge]][c-riker-crates.io] [![riker-github][c-riker-github-badge]][c-riker-github] [![riker-lib.rs][c-riker-lib.rs-badge]][c-riker-lib.rs]{{hi:riker}}{{hi:Async}}{{hi:Actors}}{{hi:Cqrs}}{{hi:Actor-model}}{{hi:Event_sourcing}}

An Actor Framework for Rust to build fast, highly concurrent and resilient applications.

```rust,editable
{{#include ../../../crates/ex/categories/c/tests/concurrency/actors.rs:example}}
```

## `ractor` {#ractor}

[![ractor][c-ractor-badge]][c-ractor]{{hi:ractor}}
[![ractor-crates.io][c-ractor-crates.io-badge]][c-ractor-crates.io]
[![ractor-github][c-ractor-github-badge]][c-ractor-github]
[![ractor-lib.rs][c-ractor-lib.rs-badge]][c-ractor-lib.rs]

```rust,editable
{{#include ../../../crates/ex/categories/c/tests/concurrency/ractor.rs:example}}
```

## `actix` {#actix}

[![actix][c-actix-badge]][c-actix]{{hi:actix}}
[![actix-crates.io][c-actix-crates.io-badge]][c-actix-crates.io]
[![actix-github][c-actix-github-badge]][c-actix-github]
[![actix-lib.rs][c-actix-lib.rs-badge]][c-actix-lib.rs]

```rust,editable
{{#include ../../../crates/ex/categories/c/tests/concurrency/actix.rs:example}}
```

## See also

[Actors with `Tokio`][blog-actors-with-tokio]{{hi:Actors}}⮳

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[_actors: organize (P2)](https://github.com/john-cd/rust_howto/issues/269)
</div>
