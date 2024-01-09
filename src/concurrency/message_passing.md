# Message passing

One increasingly popular approach to ensuring safe concurrency is message passing, where threads communicate by sending each other messages containing data. The Rust standard library provides channels for message passing that are safe to use in concurrent contexts.

Message passing in `async` programming is covered in a separate page: [async channels](async_channels.md)

## Multiple producers, single consumer

```rust,editable
{{#include ../../deps/examples/message_passing_mpsc.rs}}
```

## Crossbeam_channel

Multi-producer multi-consumer channels for message passing.

```rust,editable,ignore,mdbook-runnable
{{#include ../../deps/examples/message_passing_crossbeam_channel.rs}}
```

Example using specialized channels for tickers and timeout

```rust,editable,ignore,mdbook-runnable
{{#include ../../deps/examples/message_passing_crossbeam_channel_after_tick.rs}}
```

## Reference

[Message Passing (rust book)][message-passing-rust-book]⮳

{{#include ../link-refs.md}}
