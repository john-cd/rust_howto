# Actors in Rust

{{#include _actors.incl.md}}

An Actor is a computational entity that, in response
to a message it receives, can concurrently:
● send a finite number of messages to other Actors;
● create a finite number of new Actors;
● designate the behavior to be used for the next
message it receives.”

- Carl Hewitt 1973

An actor is a lightweight, independent unit of computation that encapsulates state and behavior. Actors communicate with each other asynchronously by exchanging messages, which are placed in the recipient's mailbox (a queue). An actor processes messages sequentially, one at a time, from its mailbox. This message-passing paradigm simplifies concurrent programming by avoiding shared mutable state and complex locking mechanisms, enabling actors to run concurrently and potentially be distributed across multiple machines, making them well-suited for building scalable and resilient systems.

Key Benefits of Using Actors:

- Simplified concurrency: Actors make it easier to write concurrent code without having to deal with low-level threading primitives.
- Improved modularity: Actors promote a modular design, making it easier to develop and maintain complex systems.
- Enhanced scalability: Actors can be easily distributed across multiple machines, allowing your system to scale horizontally.
- Increased resilience: Actors can be restarted or replicated, making your system more fault-tolerant.

Possible applications of actor frameworks:

- Web scraping: fetch data from multiple websites simultaneously,
- Game servers: represent each player or game entity by an actor,
- Microservices: implement microservices as a set of actors,
- IoT devices: represent individual devices as actors,
- Real-time analytics: process streams of data by a succession of actors,
- Chat applications,
- Telecommunications.

## `stakker` {#stakker}

[![stakker-website][c-stakker-website-badge]][c-stakker-website] [![stakker][c-stakker-badge]][c-stakker] [![stakker-crates.io][c-stakker-crates.io-badge]][c-stakker-crates.io] [![stakker-github][c-stakker-github-badge]][c-stakker-github] [![stakker-lib.rs][c-stakker-lib.rs-badge]][c-stakker-lib.rs]{{hi:stakker}}{{hi:Runtime}}{{hi:Erlang}}{{hi:Async}}{{hi:Actor}}{{hi:Pony}} [![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]{{hi:Asynchronous}} [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]{{hi:Concurrency}} [![cat-data-structures][cat-data-structures-badge]][cat-data-structures]{{hi:Data structures}}

`stakker` is a lightweight low-level single-threaded actor runtime.

```rust,editable
{{#include ../../../crates/cats/concurrency/tests/actors/stakker.rs:example}}
```

## `riker` {#riker}

[![riker-website][c-riker-website-badge]][c-riker-website] [![riker][c-riker-badge]][c-riker] [![riker-crates.io][c-riker-crates.io-badge]][c-riker-crates.io] [![riker-github][c-riker-github-badge]][c-riker-github] [![riker-lib.rs][c-riker-lib.rs-badge]][c-riker-lib.rs]{{hi:riker}}{{hi:Async}}{{hi:Actors}}{{hi:Cqrs}}{{hi:Actor-model}}{{hi:Event_sourcing}}

`riker` is an actor framework for Rust to build fast, highly concurrent, and resilient applications.

```rust,editable
{{#include ../../../crates/cats/concurrency/tests/actors/actors.rs:example}}
```

## `ractor` {#ractor}

[![ractor][c-ractor-badge]][c-ractor]{{hi:ractor}}
[![ractor-crates.io][c-ractor-crates.io-badge]][c-ractor-crates.io]
[![ractor-github][c-ractor-github-badge]][c-ractor-github]
[![ractor-lib.rs][c-ractor-lib.rs-badge]][c-ractor-lib.rs]

`ractor` is a pure-Rust actor framework, inspired from Erlang's `gen_server`.

```rust,editable
{{#include ../../../crates/cats/concurrency/tests/actors/ractor.rs:example}}
```

## `actix` {#actix}

[![actix][c-actix-badge]][c-actix]{{hi:actix}}
[![actix-crates.io][c-actix-crates.io-badge]][c-actix-crates.io]
[![actix-github][c-actix-github-badge]][c-actix-github]
[![actix-lib.rs][c-actix-lib.rs-badge]][c-actix-lib.rs]

```rust,editable
{{#include ../../../crates/cats/concurrency/tests/actors/actix.rs:example}}
```

## See also

[Actors with `Tokio`][blog-actors-with-tokio]{{hi:Actors}}⮳

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[_actors: organize (P2)](https://github.com/john-cd/rust_howto/issues/269)

TODO P2 add [ractor blog](https://slawlor.github.io/ractor/)
</div>
