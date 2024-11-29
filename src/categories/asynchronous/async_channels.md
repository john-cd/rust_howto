# Channels for use in async code

{{#include async_channels.incl.md}}

The most common form of synchronization{{hi:Synchronization}} in an async{{hi:async}} program is message passing{{hi:Message passing}}. Two tasks operate independently and send messages to each other to synchronize. Doing so has the advantage of avoiding shared state{{hi:Shared state}}. Message passing is implemented using async channels{{hi:Async channels}}.

Tokio's [`sync`][c-tokio-sync]⮳ module provides channels{{hi:Channels}} that work well with async code.

## OneShot {#oneshot}

[![tokio][c-tokio-badge]][c-tokio]{{hi:tokio}} [![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]{{hi:Asynchronous}}

[`tokio::sync::oneshot`][c-tokio::sync::oneshot]{{hi:tokio::sync::oneshot}}⮳ sends a single value from a single producer{{hi:Producer}} to a single consumer{{hi:Consumer}}. This channel{{hi:Channels}} is usually used to send the result of a computation to a waiter.

```rust,editable
{{#include ../../../deps/tests/categories/asynchronous/async_channels_oneshot.rs:example}}
```

Another example:

```rust,editable
{{#include ../../../deps/tests/categories/asynchronous/async_channels_oneshot2.rs:example}}
```

## Multiple Producer, Single Consumer {#mpsc}

[![tokio][c-tokio-badge]][c-tokio]{{hi:tokio}} [![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]{{hi:Asynchronous}}

```rust,editable
{{#include ../../../deps/tests/categories/asynchronous/async_channels_mpsc.rs:example}}
```

## See also

[![async-channel][c-async_channel-badge]][c-async_channel]{{hi:async-channel}}

{{#example async_channel}}

[![postage][c-postage-badge]][c-postage]{{hi:postage}}
[![postage-crates.io][c-postage-crates.io-badge]][c-postage-crates.io]
[![postage-github][c-postage-github-badge]][c-postage-github]
[![postage-lib.rs][c-postage-lib.rs-badge]][c-postage-lib.rs]

{{#example postage}}

Fast sync and async channel:

[![kanal][c-kanal-badge]][c-kanal]{{hi:kanal}}
[![kanal-crates.io][c-kanal-crates.io-badge]][c-kanal-crates.io]
[![kanal-github][c-kanal-github-badge]][c-kanal-github]
[![kanal-lib.rs][c-kanal-lib.rs-badge]][c-kanal-lib.rs]

{{#example kanal}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO P1 review
</div>
