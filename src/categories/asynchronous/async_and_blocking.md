# Mixing Async and Blocking Code

{{#include async_and_blocking.incl.md}}

## Calling {{i:blocking code}} from async code

[![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]

- Async code should never spend a long time without reaching an `.await`.
- Don't carelessly mix async code and synchronous, blocking calls like `std::thread::sleep(Duration::from_secs(N));`
- If you have to block the thread because of expensive CPU-bound computation, call to a synchronous IO API, use the [`spawn_blocking`][tokio::task::spawn_blocking]⮳ function, use [`rayon`][rayon]⮳, or spawn a dedicated thread.

See [Async: What is blocking? blog post][blog-what-is-blocking]⮳.

## Tokio {{i:spawn_blocking}}

[![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]

Use [`spawn_blocking`][tokio::task::spawn_blocking]⮳ to run a _small portion_ of {{i:synchronous code}}.

```rust,editable,mdbook-runnable
{{#include ../../../deps/tests/call_blocking_from_async_spawn_blocking.rs}}
```

## Using the `{{i:rayon}}` crate

[![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]

```rust,editable,mdbook-runnable
{{#include ../../../deps/tests/call_blocking_from_async_rayon.rs}}
```

### Spawn a {{i:dedicated thread}}

[![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]

If a {{i:blocking operation}} keeps running forever, you should run it on a dedicated thread.

```rust,editable,mdbook-runnable
{{#include ../../../deps/tests/call_blocking_from_async_spawn_dedicated_thread.rs}}
```

## Call async code from blocking code

[Bridging with sync code][tokio-bridging-with-sync-code]⮳  [![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]

In other cases, it may be easier to structure the application as largely {{i:synchronous}}, with smaller or logically distinct {{i:asynchronous}} portions. For instance, a GUI application might want to run the GUI code on the main thread and run a Tokio runtime next to it on another thread.

### Futures executor

[![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]

[`futures-executor`][futures-executor]⮳ includes a {{i:minimal executor}}. The [`block_on`][futures_executor::block_on]⮳ function is useful if you want to run an async function synchronously in codebase that is mostly synchronous.

```rust,editable,mdbook-runnable
{{#include ../../../deps/tests/call_async_from_blocking_futures_executor.rs}}
```

### Using the Tokio runtime directly

```rust,editable,mdbook-runnable
{{#include ../../../deps/tests/call_async_from_blocking_tokio_runtime.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
