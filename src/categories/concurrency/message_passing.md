# Message passing

One increasingly popular approach to ensuring safe concurrency is {{i:message passing}}, where threads communicate by sending each other messages containing data. The Rust standard library provides channels for message passing that are safe to use in {{i:concurrent contexts}}.

Message passing in [`async`][book-rust-reference-async]⮳ programming is covered in a separate page: [async channels](../asynchronous/async_channels.md)

{{#include message_passing.incl.md}}

## Multiple producers, single consumer

[![std][std-badge]][std]  [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]

```rust,editable
{{#include ../../../deps/tests/message_passing_mpsc.rs}}
```

## Crossbeam_channel

[![crossbeam-channel][crossbeam-channel-badge]][crossbeam-channel]  [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]

Multi-producer {{i:multi-consumer channels}} for message passing.

```rust,editable,mdbook-runnable
{{#include ../../../deps/tests/message_passing_crossbeam_channel.rs}}
```

Example using specialized channels for tickers and timeout

```rust,editable,mdbook-runnable
{{#include ../../../deps/tests/message_passing_crossbeam_channel_after_tick.rs}}
```

## See also

[![cat-concurrency][cat-concurrency-badge]][cat-concurrency]

[![crossbeam][crossbeam-badge]][crossbeam]

[![postage][postage-badge]][postage]

[Message Passing (rust book)][book-rust-message-passing]⮳

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
