# Actors in Rust

{{#include _actors.incl.md}}

"An Actor is a computational entity that, in response to a message it receives, can concurrently:

- send a finite number of messages to other Actors;
- create a finite number of new Actors;
- designate the behavior to be used for the next message it receives."

- Carl Hewitt 1973

An actor is a lightweight, independent unit of computation that encapsulates state and behavior. Actors communicate with each other asynchronously by exchanging messages, which are placed in the recipient's mailbox (a queue). An actor processes messages sequentially, one at a time, from its mailbox. This message-passing paradigm simplifies concurrent programming by avoiding shared mutable state and complex locking mechanisms, enabling actors to run concurrently and potentially be distributed across multiple machines, making them well-suited for building scalable and resilient systems.

Key Benefits of Using Actors:

- Simplified [concurrency][p-concurrency]: Actors make it easier to write concurrent code without having to deal with low-level threading primitives.
- Improved modularity: Actors promote a modular design, making it easier to develop and maintain complex systems.
- Enhanced scalability: Actors can be easily distributed across multiple machines, allowing your system to scale horizontally.
- Increased resilience: Actors can be restarted or replicated, making your system more fault-tolerant.

Possible applications of actor frameworks:

- Web [scraping][p-scraping]: fetch data from multiple websites simultaneously.
- Game servers: represent each player or game entity by an actor.
- Microservices: implement microservices as a set of actors.
- IoT devices: represent individual devices as actors.
- Real-time analytics: process [streams][p-streams] of data by a succession of actors.
- Chat applications.
- Telecommunications.

## `stakker` {#stakker}

[![stakker-website][c-stakker-website-badge]][c-stakker-website] [![stakker][c-stakker-badge]][c-stakker] [![stakker-crates.io][c-stakker-crates.io-badge]][c-stakker-crates.io] [![stakker-github][c-stakker-github-badge]][c-stakker-github] [![stakker-lib.rs][c-stakker-lib.rs-badge]][c-stakker-lib.rs]{{hi:stakker}}{{hi:Runtime}}{{hi:Erlang}}{{hi:Async}}{{hi:Actor}}{{hi:Pony}} [![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]{{hi:Asynchronous}} [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]{{hi:Concurrency}} [![cat-data-structures][cat-data-structures-badge]][cat-data-structures]{{hi:Data structures}}

[`stakker`][c-stakker]⮳{{hi:stakker}} is a lightweight low-level single-threaded actor runtime. It simplifies the development of concurrent applications by promoting message-passing between isolated actors, ensuring data safety and preventing race conditions.

```rust,editable
{{#include ../../../crates/cats/concurrency/tests/actors/stakker.rs:example}}
```

## `riker` {#riker}

[![riker-website][c-riker-website-badge]][c-riker-website] [![riker][c-riker-badge]][c-riker] [![riker-crates.io][c-riker-crates.io-badge]][c-riker-crates.io] [![riker-github][c-riker-github-badge]][c-riker-github] [![riker-lib.rs][c-riker-lib.rs-badge]][c-riker-lib.rs]{{hi:riker}}{{hi:Async}}{{hi:Actors}}{{hi:CQRS}}{{hi:Actor-model}}{{hi:Event_sourcing}}

[`riker`][c-riker]⮳{{hi:riker}} is an actor framework for Rust to build fast, highly concurrent, and resilient applications. Riker is an actor framework for Rust inspired by the actor model found in Erlang/OTP and Akka. It provides tools for creating, managing, and interacting with actors, which are lightweight, concurrent units of execution that communicate through message passing. Riker emphasizes fault tolerance and resilience, offering mechanisms for handling actor failures and ensuring the stability of concurrent applications.

```rust,editable
{{#include ../../../crates/cats/concurrency/tests/actors/actors.rs:example}}
```

## `ractor` {#ractor}

[![ractor][c-ractor-badge]][c-ractor]{{hi:ractor}}
[![ractor-crates.io][c-ractor-crates.io-badge]][c-ractor-crates.io]
[![ractor-github][c-ractor-github-badge]][c-ractor-github]
[![ractor-lib.rs][c-ractor-lib.rs-badge]][c-ractor-lib.rs]

[`ractor`][c-ractor]⮳{{hi:ractor}} is a pure-Rust actor framework, inspired from Erlang's `gen_server`.

Ractor is a Rust crate providing a simple, lightweight actor framework. It facilitates concurrent programming by enabling the creation and management of actors, which communicate through [asynchronous][p-asynchronous] message passing. Ractor focuses on ease of use and aims to provide a minimal but functional actor system, suitable for applications where a full-fledged actor framework like Riker might be overkill. It provides tools for defining actor behavior, sending and receiving messages, and handling actor lifecycles.

```rust,editable
{{#include ../../../crates/cats/concurrency/tests/actors/ractor.rs:example}}
```

## `actix` {#actix}

[![actix][c-actix-badge]][c-actix]{{hi:actix}}
[![actix-crates.io][c-actix-crates.io-badge]][c-actix-crates.io]
[![actix-github][c-actix-github-badge]][c-actix-github]
[![actix-lib.rs][c-actix-lib.rs-badge]][c-actix-lib.rs]

[`actix`][c-actix]⮳{{hi:actix}} is a performant actor framework for Rust, emphasizing message passing concurrency. It provides tools for creating, managing, and interacting with actors, which are isolated units of execution that communicate asynchronously via messages. Actix is widely used for building concurrent applications, particularly web services, due to its efficiency and support for asynchronous I/O. It offers features like message routing, supervision, and a variety of actor communication patterns.

```rust,editable
{{#include ../../../crates/cats/concurrency/tests/actors/actix.rs:example}}
```

## Related Topics {#skip}

- [[_actix | Actix]] Web.
- [[concurrency | Concurrency]].
- [[message_passing | Message Passing]].
- [[network-programming | Network Programming]].
- [[web-programming | Web Programming]].
- [[web-programming_http-server | Web Programming: HTTP Server]].

## References

- [Actors with `Tokio`][blog-actors-with-tokio]{{hi:Actors}}⮳.
- [`ractor` blog][ractor blog][c-ractor-blog]⮳.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[_actors: organize](https://github.com/john-cd/rust_howto/issues/269)
</div>
