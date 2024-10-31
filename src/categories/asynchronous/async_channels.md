# Channels for use in async code

The most common form of synchronization{{hi:Synchronization}} in an async{{hi:async}} program is Message passing{{hi:Message passing}}. Two tasks operate independently and send messages to each other to synchronize. Doing so has the advantage of avoiding shared state{{hi:Shared state}}. Message passing is implemented using async channels{{hi:Async channels}}.

Tokio's [`sync`][c-tokio-sync]⮳ module provides channels{{hi:Channels}} that work well with async code.

{{#include async_channels.incl.md}}

## OneShot

[![tokio][c-tokio-badge]][c-tokio]{{hi:tokio}}  [![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]{{hi:Asynchronous}}

[`tokio::sync::oneshot`][c-tokio::sync::oneshot]{{hi:tokio::sync::oneshot}}⮳ sends a single value from a single producer{{hi:Producer}} to a single consumer{{hi:Consumer}}. This channel{{hi:Channels}} is usually used to send the result of a computation to a waiter.

```rust
{{#include ../../../deps/tests/cats/asynchronous/async_channels_oneshot.rs}}
```

Another example:

```rust
{{#include ../../../deps/tests/cats/asynchronous/async_channels_oneshot2.rs}}
```

## Multiple Producer, Single Consumer

[![tokio][c-tokio-badge]][c-tokio]{{hi:tokio}}  [![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]{{hi:Asynchronous}}

```rust
{{#include ../../../deps/tests/cats/asynchronous/async_channels_mpsc.rs}}
```

## See also

[![async-channel][c-async_channel-badge]][c-async_channel]{{hi:async-channel}}

[![postage][c-postage-badge]][c-postage]{{hi:postage}}  [![postage-lib.rs][c-postage-lib.rs-badge]][c-postage-lib.rs]

Fast sync and async channel: [Kanal][c-kanal-github]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO: organize see also
</div>
