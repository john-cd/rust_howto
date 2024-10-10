# Channels for use in async code

The most common form of {{i:synchronization}} in an {{i:async}} program is {{i:message passing}}. Two tasks operate independently and send messages to each other to synchronize. Doing so has the advantage of {{i:avoiding shared state}}. Message passing is implemented using {{i:async channels}}.

Tokio's [`{{i:sync}}`][c-tokio-sync]⮳ module provides {{i:channels}} that work well with async code.

{{#include async_channels.incl.md}}

## OneShot

[![tokio][c-tokio-badge]][c-tokio]  [![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]

[`{{i:oneshot}}`][c-tokio::sync::oneshot]⮳ sends a single value from a single {{i:producer}} to a single {{i:consumer}}. This {{i:channel}} is usually used to send the result of a computation to a waiter.

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

[![async-channel][async-channel-badge]][async-channel]

[![postage][postage-badge]][postage]  [![postage-librs][postage-librs-badge]][postage-librs]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
