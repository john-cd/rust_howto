# Channels for use in async code

The most common form of synchronization{{hi:Synchronization}} in an async{{hi:async}} program is message passing{{hi:Message passing}}. Two tasks operate independently and send messages to each other to synchronize. Doing so has the advantage of avoiding shared state{{hi:Shared state}}. Message passing is implemented using async channels{{hi:Async channels}}.

Tokio's [`sync`][c-tokio-sync]⮳ module provides channels{{hi:Channel}} that work well with async code.

{{#include async_channels.incl.md}}

## OneShot

[![tokio][c-tokio-badge]][c-tokio]  [![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]

[`tokio::sync::oneshot`][c-tokio::sync::oneshot]{{hi:tokio::sync::oneshot}}⮳ sends a single value from a single producer{{hi:Producer}} to a single consumer{{hi:Consumer}}. This channel{{hi:Channels}} is usually used to send the result of a computation to a waiter.

```rust,editable,mdbook-runnable
{{#include ../../../deps/tests/async_channels_oneshot.rs}}
```

Another example:

```rust,editable,mdbook-runnable
{{#include ../../../deps/tests/async_channels_oneshot2.rs}}
```

## Multiple Producer, Single Consumer

[![tokio][c-tokio-badge]][c-tokio]  [![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]

```rust,editable,mdbook-runnable
{{#include ../../../deps/tests/async_channels_mpsc.rs}}
```

## See also

[![async_channel][c-async_channel-badge]][c-async_channel]

[![postage][c-postage-badge]][c-postage]  [![postage-lib.rs][c-postage-lib.rs-badge]][c-postage-lib.rs]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO:
</div>
