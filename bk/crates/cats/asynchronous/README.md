<!-- TODO Reviewed ??-??-2026 -->

# Asynchronous

Book code examples for Asynchronous programming with Tokio, Smol, and Rayon. It demonstrates integration with key ecosystem crates including `anyhow`, `async-channel`, `async-trait`, `clap`, `futures`, `futures-executor`.

## Examples Covered

- **Async** (`examples/async/`): Demonstrates concepts like `async1`, `async2`, `async3`.
- **Async Channels** (`examples/async_channels/`): Demonstrates concepts like `async_channel`, `async_channels_mpsc`, `async_channels_oneshot`, `async_channels_oneshot2`, `kanal`, `postage`.
- **Async Traits** (`examples/async_traits/`): Demonstrates concepts like `afit`, `async_trait_dyn`, `async_traits`, `async_traits2`, `async_traits3`, `rpitit_send`.
- **Call Async From Blocking** (`examples/call_async_from_blocking/`): Demonstrates concepts like `call_async_from_blocking_futures_executor`, `call_async_from_blocking_tokio_runtime`.
- **Call Blocking From Async** (`examples/call_blocking_from_async/`): Demonstrates concepts like `call_blocking_from_async_rayon`, `call_blocking_from_async_spawn_blocking`, `call_blocking_from_async_spawn_dedicated_thread`.
- **Futures** (`examples/futures/`): Demonstrates concepts like `futures1`, `futures2`, `futures3`, `futures_util`.
- **Streams** (`examples/streams/`): Demonstrates concepts like `streams`, `streams2`, `streams3`.
- **Structured Concurrency** (`examples/structured_concurrency/`): Demonstrates concepts like `cancellation_tree`, `joinset_pool`, `supervisor`, `watch_latest`.
- **Tokio** (`examples/tokio/`): Demonstrates concepts like `tokio2`, `tokio21`, `tokio_graceful_shutdown`.
- **Standalone Examples**: `afit`, `async_trait_dyn`, `cancellation_token`, `cancellation_tree`, `futures_unordered`, `joinset_pool`, `mio`, `rpitit_send`, `smol`, `supervisor`, `watch_latest`.

## Usage

To check or run examples in this crate:

```console
cargo check --package asynchronous --examples
cargo test --package asynchronous
```

Code examples correspond to chapters in `bk/src/categories/asynchronous` or related book sections.
