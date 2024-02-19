# Channels for use in async code

The most common form of synchronization in an async program is message passing. Two tasks operate independently and send messages to each other to synchronize. Doing so has the advantage of avoiding shared state. Message passing is implemented using channels.

Tokio's `sync` module provides channels that work well with async code.

## OneShot

[![tokio-badge]][tokio]

`oneshot` sends a single value from a single producer to a single consumer. This channel is usually used to send the result of a computation to a waiter.

```rust,editable,mdbook-runnable
{{#include ../../deps/examples/async_channels_oneshot.rs}}
```

Another example:

```rust,editable,mdbook-runnable
{{#include ../../deps/examples/async_channels_oneshot2.rs}}
```

## Multiple Producer, Single Consumer

[![tokio-badge]][tokio]

```rust,editable,mdbook-runnable
{{#include ../../deps/examples/async_channels_mpsc.rs}}
```

## See also

[![async-channel-badge]][async-channel]

[![postage-badge]][postage] [(lib.rs)][postage-librs]

{{#include ../refs/link-refs.md}}
