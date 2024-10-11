# Mixing Async and Blocking Code

{{#include async_and_blocking.incl.md}}

## Calling blocking code{{hi:blocking code}} from async code

[![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]

- Async code should never spend a long time without reaching an `.await`.
- Don't carelessly mix async{{hi:async}} code and synchronous, blocking calls like `std::thread::sleep(Duration::from_secs(N));`
- If you have to block the thread because of expensive CPU-bound{{hi:CPU-bound}} computation, call to a synchronous{{hi:synchronous}} IO API, use the [`tokio::task::spawn_blocking`][c-tokio::task::spawn_blocking]{{hi:tokio::task::spawn_blocking}}⮳ function, use [`rayon`][c-rayon]{{hi:rayon}}⮳, or spawn a dedicated thread{{hi:dedicated thread}}.

See [Async: What is blocking? blog post][blog-what-is-blocking]⮳.

## Tokio spawn_blocking{{hi:spawn_blocking}}

[![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]

Use [`tokio::task::spawn_blocking`][c-tokio::task::spawn_blocking]{{hi:tokio::task::spawn_blocking}}⮳ to run a _small portion_ of synchronous code{{hi:synchronous code}}.

```rust,editable,mdbook-runnable
{{#include ../../../deps/tests/call_blocking_from_async_spawn_blocking.rs}}
```

## Using the `rayon`{{hi:rayon}} crate

[![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]

```rust,editable,mdbook-runnable
{{#include ../../../deps/tests/call_blocking_from_async_rayon.rs}}
```

### Spawn a dedicated thread{{hi:dedicated thread}}

[![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]

If a blocking operation{{hi:blocking operation}} keeps running forever, you should run it on a dedicated thread{{hi:dedicated thread}}.

```rust,editable,mdbook-runnable
{{#include ../../../deps/tests/call_blocking_from_async_spawn_dedicated_thread.rs}}
```

## Call async code from blocking code

[Bridging with sync code][c-tokio_bridging_with_sync_code-website]⮳  [![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]

In other cases, it may be easier to structure the application as largely synchronous{{hi:synchronous}}, with smaller or logically distinct asynchronous{{hi:asynchronous}} portions. For instance, a GUI{{hi:GUI}} application might want to run the GUI code on the main thread and run a Tokio runtime next to it on another thread.

### Futures executor

[![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]

[`futures_executor`][c-futures_executor]{{hi:futures_executor}}⮳ includes a minimal executor [`futures_executor::block_on`][c-futures_executor::block_on]{{hi:futures_executor::block_on}}⮳ function is useful if you want to run an async function synchronously in codebase that is mostly synchronous.

```rust,editable,mdbook-runnable
{{#include ../../../deps/tests/call_async_from_blocking_futures_executor.rs}}
```

### Using the Tokio runtime directly

```rust,editable,mdbook-runnable
{{#include ../../../deps/tests/call_async_from_blocking_tokio_runtime.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
