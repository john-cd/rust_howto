# Message passing

One increasingly popular approach to ensuring safe concurrency is message passing, where threads communicate by sending each other messages containing data. The Rust standard library provides channels for message passing that are safe to use in concurrent contexts.

Message passing in `async` programming is covered in a separate page: [async channels](../async/async_channels.md)

## Multiple producers, single consumer

[![std-badge]][std]

```rust,editable
{{#include ../../deps/tests/message_passing_mpsc.rs}}
```

## Crossbeam_channel

[![crossbeam-channel-badge]][crossbeam-channel]

Multi-producer multi-consumer channels for message passing.

```rust,editable,mdbook-runnable
{{#include ../../deps/tests/message_passing_crossbeam_channel.rs}}
```

Example using specialized channels for tickers and timeout

```rust,editable,mdbook-runnable
{{#include ../../deps/tests/message_passing_crossbeam_channel_after_tick.rs}}
```

## See also

[![crossbeam-badge]][crossbeam]

[![postage-badge]][postage]

[Message Passing (rust book)][book-rust-message-passing]⮳

{{#include ../refs/link-refs.md}}
