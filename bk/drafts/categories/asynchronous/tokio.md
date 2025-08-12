# Tokio

{{#include tokio.incl.md}}

## Basics {#basics}

[![tokio~website][c~tokio~website~badge]][c~tokio~website] [![tokio][c~tokio~docs~badge]][c~tokio~docs] [![tokio~crates.io][c~tokio~crates.io~badge]][c~tokio~crates.io] [![tokio~github][c~tokio~github~badge]][c~tokio~github] [![tokio~lib.rs][c~tokio~lib.rs~badge]][c~tokio~lib.rs]{{hi:tokio}}{{hi:Async}}{{hi:Futures}}{{hi:Io}}{{hi:Non-blocking}} [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}}

Tokio{{hi:tokio}} is an asynchronous runtime{{hi:Asynchronous runtime}} for the Rust programming language. It provides the building blocks needed for writing networking applications{{hi:Networking applications}}. Tokio provides a few major components:

- Multiple variations of the runtime for executing [asynchronous][p~async] code. Everything from a multi-threaded{{hi:Multithreading}}, work-stealing runtime{{hi:Work-stealing runtime}} to a light-weight, single-threaded runtime{{hi:Single-threaded runtime}}.
- An [asynchronous][p~async] version of the [standard library][p~standard-library].
- A large ecosystem of libraries.

- creating and running a runtime{{hi:Runtime}}, spawning tasks{{hi:Spawning tasks}}, working with I/O and timers, and handling errors.

### Join {#join}

By running all [async][p~async]{{hi:async}} expressions on the current task, the expressions are able to run concurrently but not in parallel. This means all expressions are run on the same thread and if one branch blocks the thread, all other expressions will be unable to continue. If parallelism is required, spawn each async expression using `tokio::spawn`{{hi:tokio::spawn}} and pass the join handle to `join!`{{hi:join!}}.

### Spawning {#spawning}

## IO {#io}

Read and write data asynchronously with Tokio, using [streams][p~streams], codecs, and [futures][p~futures]. It also shows how to handle errors and timeouts.

[Current thread runtime][c~tokio::main~current-thread-runtime~docs]{{hi:tokio::main}}↗.

equivalent to

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/tokio/tokio2.rs:example}}
```

[LocalSet][c~tokio::task::LocalSet~docs]{{hi:tokio::task::LocalSet}}↗.

In some cases, it is necessary to run one or more [futures][p~futures] that do not implement Send{{hi:Send}} and thus are unsafe to send between threads. In these cases, a local task set may be used to schedule one or more `!Send` futures to run together on the same thread.

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/tokio/tokio21.rs:example}}
```

## Graceful Shutdown {#graceful-shutdown}

[![tokio-graceful-shutdown][c~tokio-graceful-shutdown~docs~badge]][c~tokio-graceful-shutdown~docs] [![tokio-graceful-shutdown~crates.io][c~tokio-graceful-shutdown~crates.io~badge]][c~tokio-graceful-shutdown~crates.io] [![tokio-graceful-shutdown~github][c~tokio-graceful-shutdown~github~badge]][c~tokio-graceful-shutdown~github] [![tokio-graceful-shutdown~lib.rs][c~tokio-graceful-shutdown~lib.rs~badge]][c~tokio-graceful-shutdown~lib.rs]{{hi:tokio-graceful-shutdown}}{{hi:Shutdown}}{{hi:Tokio}} [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}}

Example from [tokio-graceful-shutdown][c~tokio-graceful-shutdown~docs]{{hi:tokio-graceful-shutdown}}↗:

```rust,editable,noplayground
{{#include ../../../crates/cats/asynchronous/examples/tokio/tokio_graceful_shutdown.rs:example}}
```

## Useful Links

- [`tokio.rs`][c~tokio~website]↗.
- [![tokio examples][c~tokio~examples~docs~badge]][c~tokio~examples~docs]{{hi:Tokio examples}}.
- [tokio-rs `async-stream`][c~async-stream~github]↗.
- [tokio-rs `mio`][c~mio~github]↗.
- [`tokio` glossary][c~tokio~glossary~website]↗.
- [`tokio` tutorial][c~tokio~tutorial~docs]{{hi:tokio}}↗.
- Tokio "mini-Redis" example: [![tokio~mini-redis][c~tokio~mini-redis~github~badge]][c~tokio~mini-redis~github].
- Template for a tokio-rs app with logging & command line argument parser: [![rust-tokio-template~github][rust-tokio-template~github~badge]][rust-tokio-template~github].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[tokio: review](https://github.com/john-cd/rust_howto/issues/223)
</div>
