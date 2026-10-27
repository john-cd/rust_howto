<!-- TODO Reviewed ??-??-2026 -->

# Actors in Rust

{{#include _actors.incl.md}}

## Actor Model {#actor-model}

An actor owns its state and processes messages from a mailbox one at a time. It can send messages to other actors and create actors; this message-passing model lets independent tasks coordinate without sharing mutable state.

## Build Actors with Tokio {#tokio}

A Tokio task and channels can implement a small actor without an actor-framework dependency. The task owns the counter, a bounded `mpsc` channel carries commands, and each command uses a `oneshot` channel to return its reply. See [Actors with Tokio][blog~actors-with-tokio]↗ for a fuller explanation.

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/actors/actors.rs:example}}
```

## `actix` {#actix}

[![actix][c~actix~docs~badge]][c~actix~docs] [![actix~crates.io][c~actix~crates.io~badge]][c~actix~crates.io] [![actix~repo][c~actix~repo~badge]][c~actix~repo] [![actix~lib.rs][c~actix~lib.rs~badge]][c~actix~lib.rs]

[`actix`][c~actix~docs]↗ uses typed messages and an actor system for synchronous or asynchronous message handling, with supervision support.

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/actors/actix.rs:example}}
```

## `ractor` {#ractor}

[![ractor][c~ractor~docs~badge]][c~ractor~docs] [![ractor~crates.io][c~ractor~crates.io~badge]][c~ractor~crates.io] [![ractor~repo][c~ractor~repo~badge]][c~ractor~repo] [![ractor~lib.rs][c~ractor~lib.rs~badge]][c~ractor~lib.rs]

[`ractor`][c~ractor~docs]↗ is an Erlang-inspired actor framework with actor lifecycle and supervision primitives, and support for Tokio or async-std runtimes.

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/actors/ractor.rs:example}}
```

## `kameo` {#kameo}

[![kameo][c~kameo~docs~badge]][c~kameo~docs] [![kameo~crates.io][c~kameo~crates.io~badge]][c~kameo~crates.io] [![kameo~repo][c~kameo~repo~badge]][c~kameo~repo] [![kameo~lib.rs][c~kameo~lib.rs~badge]][c~kameo~lib.rs]

[`kameo`][c~kameo~docs]↗ provides asynchronous actors with supervision and fault-recovery support; see its guide for examples.

## `stakker` {#stakker}

[![stakker~website][c~stakker~website~badge]][c~stakker~website] [![stakker][c~stakker~docs~badge]][c~stakker~docs] [![stakker~crates.io][c~stakker~crates.io~badge]][c~stakker~crates.io] [![stakker~repo][c~stakker~repo~badge]][c~stakker~repo] [![stakker~lib.rs][c~stakker~lib.rs~badge]][c~stakker~lib.rs]

[`stakker`][c~stakker~docs]↗ is a lightweight, single-threaded actor runtime that uses deferred calls and callbacks for message replies.

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/actors/stakker.rs:example}}
```

## Comparison

| Option                  | Runtime and model                                                          | Local example                                                               |
| ----------------------- | -------------------------------------------------------------------------- | --------------------------------------------------------------------------- |
| Tokio task and channels | Tokio task with an explicit mailbox and request/reply messages             | [`actors.rs`](../../../crates/cats/concurrency/examples/actors/actors.rs)   |
| Actix                   | Actor system with typed messages, sync and async handling, and supervision | [`actix.rs`](../../../crates/cats/concurrency/examples/actors/actix.rs)     |
| Ractor                  | Erlang-inspired actor lifecycle and supervision; Tokio or async-std        | [`ractor.rs`](../../../crates/cats/concurrency/examples/actors/ractor.rs)   |
| Kameo                   | Asynchronous actors with supervision and fault recovery                    | [Kameo guide][c~kameo~docs]↗                                                |
| Xtra (low activity)     | Small asynchronous actors that can use different futures executors         | [Xtra guide][c~xtra~docs]↗                                                  |
| Stakker                 | Lightweight, single-threaded actors with deferred calls and callbacks      | [`stakker.rs`](../../../crates/cats/concurrency/examples/actors/stakker.rs) |

## Historical and Unmaintained Frameworks {#historical-frameworks}

`bastion` and `riker` are retained here for historical context; their upstream repositories have had no recent development activity. `xtra` is not archived, but its latest upstream commit is dated November 16, 2024, so check maintenance and compatibility before adopting it. Riker is disabled in the example workspace because its dependency chain includes vulnerable, outdated crates.

### `bastion`

[`bastion`][c~bastion~repo]↗ was a distributed, fault-tolerant actor runtime.

### `riker` {#riker}

[`riker`][c~riker~repo]↗ was inspired by Erlang/OTP and Akka; its source is kept in the repository but its feature and dependency remain disabled.

### `xtra` {#xtra}

[`xtra`][c~xtra~repo]↗ is a small asynchronous actor framework whose handlers can run on different futures executors.

## References {#references .skip}

- [Actors with Tokio][blog~actors-with-tokio]↗{{hi:Actors}}.
- [`ractor` blog][ractor blog][c~ractor~blog]↗.

## Related Topics {#related-topics .skip}

- [[_actix | Actix]] Web.
- [[asynchronous | Asynchronous Programming]].
- [[concurrency | Concurrency]].
- [[message_passing | Message Passing]].
- [[network-programming | Network Programming]].
- [[web-programming | Web Programming]].
- [[web-programming_http-server | Web Programming: HTTP Server]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
