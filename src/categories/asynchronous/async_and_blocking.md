# Mixing Async and Blocking Code

{{#include async_and_blocking.incl.md}}

## Calling blocking code from async code

[![tokio][c-tokio-badge]][c-tokio]{{hi:tokio}}  [![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]{{hi:Asynchronous}}{{hi:Blocking code}}

- Async code should never spend a long time without reaching an `.await`.
- Don't carelessly mix async{{hi:async}} code and synchronous, blocking calls like `std::thread::sleep(Duration::from_secs(N));`
- If you have to block the thread because of expensive CPU-bound{{hi:CPU bound}} computation, call to a synchronous IO API{{hi:Synchronous IO}}, use the [`tokio::task::spawn_blocking`][c-tokio::task::spawn_blocking]{{hi:tokio::task::spawn_blocking}}⮳ function, use [`rayon`][c-rayon]{{hi:rayon}}⮳, or spawn a dedicated thread{{hi:Dedicated thread}}.

See [Async: What is blocking? blog post][blog-async-what-is-blocking]⮳.

## Tokio spawn_blocking

[![tokio][c-tokio-badge]][c-tokio]{{hi:tokio}}  [![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]{{hi:Asynchronous}}{{hi:spawn_blocking}}

Use [`tokio::task::spawn_blocking`][c-tokio::task::spawn_blocking]{{hi:tokio::task::spawn_blocking}}⮳ to run a _small portion_ of synchronous code{{hi:Synchronous code}}.

```rust
{{#include ../../../deps/tests/cats/asynchronous/call_blocking_from_async_spawn_blocking.rs}}
```

## Using the `rayon` crate

[![rayon][c-rayon-badge]][c-rayon]{{hi:rayon}}  [![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]{{hi:Asynchronous}}{{hi:rayon}}

```rust
{{#include ../../../deps/tests/cats/asynchronous/call_blocking_from_async_rayon.rs}}
```

### Spawn a dedicated thread

[![rayon][c-rayon-badge]][c-rayon]{{hi:rayon}}  [![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]{{hi:Asynchronous}}{{hi:Dedicated thread}}

If a blocking operation{{hi:Blocking operation}} keeps running forever, you should run it on a dedicated thread{{hi:Dedicated thread}}.

```rust
{{#include ../../../deps/tests/cats/asynchronous/call_blocking_from_async_spawn_dedicated_thread.rs}}
```

## Call async code from blocking code

[Bridging with sync code][c-tokio_bridging_with_sync_code-website]⮳  [![tokio][c-tokio-badge]][c-tokio]{{hi:tokio}} [![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]{{hi:Asynchronous}}

In other cases, it may be easier to structure the application as largely synchronous{{hi:Synchronous code}}, with smaller or logically distinct asynchronous{{hi:Asynchronous}} portions. For instance, a GUI{{hi:GUI}} application might want to run the GUI code on the main thread and run a Tokio runtime next to it on another thread.

### Futures executor

[![futures_executor][c-futures_executor-badge]][c-futures_executor]{{hi:futures_executor}}  [![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]{{hi:Asynchronous}}

[`futures_executor`][c-futures_executor]{{hi:futures_executor}}⮳ includes a minimal executor. The [`futures_executor::block_on`][c-futures_executor::block_on]{{hi:futures_executor::block_on}}⮳ function is useful if you want to run an async function synchronously in codebase that is mostly synchronous.

```rust
{{#include ../../../deps/tests/cats/asynchronous/call_async_from_blocking_futures_executor.rs}}
```

### Using the Tokio runtime directly

[![tokio][c-tokio-badge]][c-tokio]{{hi:tokio}}  [![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]{{hi:Asynchronous}}

```rust
{{#include ../../../deps/tests/cats/asynchronous/call_async_from_blocking_tokio_runtime.rs}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
