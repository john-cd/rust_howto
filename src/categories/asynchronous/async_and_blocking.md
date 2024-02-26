# Mixing Async and Blocking Code

{{#include async_and_blocking.incl.md}}

## Calling blocking code from async code

- Async code should never spend a long time without reaching an `.await`.
- Don't carelessly mix async code and synchronous, blocking calls like `std::thread::sleep(Duration::from_secs(N));`
- If you have to block the thread because of expensive CPU-bound computation, call to a synchronous IO API, use the `spawn_blocking` function, use `rayon`, or spawn a dedicated thread.

See [Async: What is blocking? blog post][blog-what-is-blocking]⮳.

## Tokio spawn_blocking

Use [`spawn_blocking`][tokio::task::spawn_blocking]⮳ to run a _small portion_ of synchronous code.

```rust,editable,mdbook-runnable
{{#include ../../../deps/tests/call_blocking_from_async_spawn_blocking.rs}}
```

## Using the `rayon` crate

```rust,editable,mdbook-runnable
{{#include ../../../deps/tests/call_blocking_from_async_rayon.rs}}
```

### Spawn a dedicated thread

If a blocking operation keeps running forever, you should run it on a dedicated thread.

```rust,editable,mdbook-runnable
{{#include ../../../deps/tests/call_blocking_from_async_spawn_dedicated_thread.rs}}
```

## Call async code from blocking code

[Bridging with sync code][tokio-bridging-with-sync-code]⮳

In other cases, it may be easier to structure the application as largely synchronous, with smaller or logically distinct asynchronous portions. For instance, a GUI application might want to run the GUI code on the main thread and run a Tokio runtime next to it on another thread.

### Futures executor

[`futures-executor`][futures-executor]⮳ includes a minimal executor. The [`block_on`][futures_executor::block_on]⮳ function is useful if you want to run an async function synchronously in codebase that is mostly synchronous.

```rust,editable,mdbook-runnable
{{#include ../../../deps/tests/call_async_from_blocking_futures_executor.rs}}
```

### Using the Tokio runtime directly

```rust,editable,mdbook-runnable
{{#include ../../../deps/tests/call_async_from_blocking_tokio_runtime.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
