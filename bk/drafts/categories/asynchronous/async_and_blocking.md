# Mixing Async and Blocking Code

{{#include async_and_blocking.incl.md}}

## Call Blocking Code from Async Code {#calling-blocking-from-async}

[![tokio][c~tokio~docs~badge]][c~tokio~docs]{{hi:tokio}} [![tokio~crates.io][c~tokio~crates.io~badge]][c~tokio~crates.io]
[![tokio~github][c~tokio~github~badge]][c~tokio~github] [![tokio~lib.rs][c~tokio~lib.rs~badge]][c~tokio~lib.rs] [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}}{{hi:Blocking code}}

- [Async][p~async] code should never spend a long time without reaching an [`.await`][book~rust-reference~await]⮳{{hi:.await}} [`.await`][c~await_tree~docs]⮳{{hi:.await}} .
- Don't carelessly mix [async][p~async]{{hi:async}} code and synchronous, blocking calls like `std::thread::sleep(Duration::from_secs(N));`.
- If you have to block the thread because of expensive CPU-bound{{hi:CPU bound}} computation, call to a synchronous IO API{{hi:Synchronous IO}}, use the [`tokio::task::spawn_blocking`][c~tokio::task::spawn_blocking~docs]{{hi:tokio::task::spawn_blocking}}⮳ function, use [`rayon`][c~rayon~docs]{{hi:rayon}}⮳, or spawn a dedicated thread{{hi:Dedicated thread}}.

See [Async: What is blocking? blog post][blog~async-what-is-blocking]⮳.

## Use `spawn_blocking` {#spawn-blocking}

[![tokio][c~tokio~docs~badge]][c~tokio~docs]{{hi:tokio}} [![tokio~crates.io][c~tokio~crates.io~badge]][c~tokio~crates.io]
[![tokio~github][c~tokio~github~badge]][c~tokio~github] [![tokio~lib.rs][c~tokio~lib.rs~badge]][c~tokio~lib.rs] [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}}{{hi:spawn_blocking}}

Use [`tokio::task::spawn_blocking`][c~tokio::task::spawn_blocking~docs]{{hi:tokio::task::spawn_blocking}}⮳ to run a _small portion_ of synchronous code{{hi:Synchronous code}}.

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/call_blocking_from_async/call_blocking_from_async_spawn_blocking.rs:example}}
```

## Use the `rayon` Crate {#rayon}

[![rayon][c~rayon~docs~badge]][c~rayon~docs]{{hi:rayon}} [![rayon~crates.io][c~rayon~crates.io~badge]][c~rayon~crates.io]
[![rayon~github][c~rayon~github~badge]][c~rayon~github] [![rayon~lib.rs][c~rayon~lib.rs~badge]][c~rayon~lib.rs]
[![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}}{{hi:rayon}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}}

[`rayon`][c~rayon~docs]⮳{{hi:rayon}}

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/call_blocking_from_async/call_blocking_from_async_rayon.rs:example}}
```

### Spawn a Dedicated Thread {#spawn-a-dedicated-thread}

[![rayon][c~rayon~docs~badge]][c~rayon~docs]{{hi:rayon}} [![rayon~crates.io][c~rayon~crates.io~badge]][c~rayon~crates.io]
[![rayon~github][c~rayon~github~badge]][c~rayon~github] [![rayon~lib.rs][c~rayon~lib.rs~badge]][c~rayon~lib.rs]
[![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}}{{hi:rayon}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}}{{hi:Dedicated thread}}

[`rayon`][c~rayon~docs]⮳{{hi:rayon}}

If a blocking operation{{hi:Blocking operation}} keeps running forever, you should run it on a dedicated thread{{hi:Dedicated thread}}.

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/call_blocking_from_async/call_blocking_from_async_spawn_dedicated_thread.rs:example}}
```

## Call Async Code from Blocking Code {#call-async-from-blocking}

[Bridging with sync code][c~tokio_bridging_with_sync_code~website]⮳ [![tokio][c~tokio~docs~badge]][c~tokio~docs]{{hi:tokio}} [![tokio~crates.io][c~tokio~crates.io~badge]][c~tokio~crates.io]
[![tokio~github][c~tokio~github~badge]][c~tokio~github] [![tokio~lib.rs][c~tokio~lib.rs~badge]][c~tokio~lib.rs] [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}}{{hi:spawn_blocking}}

In other cases, it may be easier to structure the application as largely synchronous{{hi:Synchronous code}}, with smaller or logically distinct asynchronous{{hi:Asynchronous}} portions. For instance, a GUI{{hi:GUI}} application might want to run the GUI code on the main thread and run a Tokio runtime next to it on another thread.

### Use the `futures` Executor {#futures-executor}

[![futures_executor][c~futures_executor~docs~badge]][c~futures_executor~docs]{{hi:futures_executor}}
[![futures_executor~crates.io][c~futures_executor~crates.io~badge]][c~futures_executor~crates.io]
[![futures_executor~github][c~futures_executor~github~badge]][c~futures_executor~github]
[![futures_executor~lib.rs][c~futures_executor~lib.rs~badge]][c~futures_executor~lib.rs] [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}}

[`futures_executor`][c~futures_executor~docs]{{hi:futures_executor}}⮳ includes a minimal executor. The [`futures_executor::block_on`][c~futures_executor::block_on~docs]{{hi:futures_executor::block_on}}⮳ function is useful if you want to run an async function synchronously in codebase that is mostly synchronous.

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/call_async_from_blocking/call_async_from_blocking_futures_executor.rs:example}}
```

### Use the Tokio Runtime Directly {#tokio-runtime}

[![tokio][c~tokio~docs~badge]][c~tokio~docs]{{hi:tokio}} [![tokio~crates.io][c~tokio~crates.io~badge]][c~tokio~crates.io]
[![tokio~github][c~tokio~github~badge]][c~tokio~github] [![tokio~lib.rs][c~tokio~lib.rs~badge]][c~tokio~lib.rs] [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}} [![cat~network-programming][cat~network-programming~badge]][cat~network-programming]{{hi:Network programming}}{{hi:spawn_blocking}}

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/call_async_from_blocking/call_async_from_blocking_tokio_runtime.rs:example}}
```

## Related Topics {#skip}

- [[concurrency | Concurrency]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[async_and_blocking: cleanup](https://github.com/john-cd/rust_howto/issues/213)
</div>
