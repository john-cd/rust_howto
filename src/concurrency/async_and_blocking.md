# Mixing Async and Blocking Code

## Calling blocking code from async code

- Async code should never spend a long time without reaching an `.await`.
- Don't carelessly mix async code and synchronous, blocking calls like `std::thread::sleep(Duration::from_secs(N));`
- If you have to block the thread because of expensive CPU-bound computation, call to a synchronous IO API, use the `spawn_blocking` function, use `rayon`, or spawn a dedicated thread.

See [Async: What is blocking? blog post]( https://ryhl.io/blog/async-what-is-blocking/ )⮳.

## Tokio spawn_blocking

Use [`spawn_blocking`]( https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html )⮳ to run a _small portion_ of synchronous code.

```rust,editable,ignore,mdbook-runnable
{{#include ../../deps/examples/call_blocking_from_async_spawn_blocking.rs}}
```

## Using the `rayon` crate

```rust,editable,ignore,mdbook-runnable
{{#include ../../deps/examples/call_blocking_from_async_rayon.rs}}
```

### Spawn a dedicated thread

If a blocking operation keeps running forever, you should run it on a dedicated thread.

```rust,editable,ignore,mdbook-runnable
{{#include ../../deps/examples/call_blocking_from_async_spawn_dedicated_thread.rs}}
```

## Call async code from blocking code

[Bridging with sync code]( https://tokio.rs/tokio/topics/bridging )⮳

In other cases, it may be easier to structure the application as largely synchronous, with smaller or logically distinct asynchronous portions. For instance, a GUI application might want to run the GUI code on the main thread and run a Tokio runtime next to it on another thread.

### Futures executor

[futures-executor][futures-executor]⮳ includes a minimal executor. The [`block_on`]( https://docs.rs/futures-executor/latest/futures_executor/fn.block_on.html )⮳ function is useful if you want to run an async function synchronously in codebase that is mostly synchronous.

```rust,editable,ignore,mdbook-runnable
{{#include ../../deps/examples/call_async_from_blocking_futures_executor.rs}}
```

### Using the Tokio runtime directly

```rust,editable,ignore,mdbook-runnable
{{#include ../../deps/examples/call_async_from_blocking_tokio_runtime.rs}}
```

{{#include ../links.md}}
