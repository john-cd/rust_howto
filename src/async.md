# Asynchronous programming

Helps you deal with events independently of the main program flow, using techniques like futures, promises, waiting, or eventing.

- Ability to make progress on multiple tasks, even if they don't execute at the exact same time.
- Mechanism: _cooperative_ multitasking - tasks yield control, allowing other tasks to run.
- Involves context switching on a single thread or, most often, among a few threads (the pool of which is opaquely managed by the async runtime).
- Achieves non-blocking I/O operations to improve responsiveness and efficiency.
- Lower overhead compared to multithreading.
- Multi-threaded async programming also requires careful synchronization to prevent data races.

Key constructs in Rust:

- `async` / `await` keywords
- `Future`s

Here are the topics we’ll cover:

- [Async](concurrency/async.md)
  - [Async and traits](async/async_traits.md)
  - [Tokio async runtime](async/tokio.md)
  - [Async channels](async/async_channels.md)
  - [Streams](async/streams.md)
  - [Futures crate](async/futures.md)
