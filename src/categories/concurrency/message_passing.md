# Message passing

One increasingly popular approach to ensuring safe concurrency is Message passing{{hi:Message passing}}, where threads communicate by sending each other messages{{hi:Messages}} containing data. The Rust standard library provides channels{{hi:Channels}} for Message passing that are safe to use in concurrent contexts{{hi:Concurrent contexts}}.

Message passing in [`async`][book-rust-reference-async]{{hi:async}}⮳ programming is covered in a separate page: [async channels][p-async-channels]{{hi:Async channels}}

{{#include message_passing.incl.md}}

## Multiple producers, single consumer

[![std][c-std-badge]][c-std]{{hi:std}}  [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]{{hi:Concurrency}}

```rust
{{#include ../../../deps/tests/cats/concurrency/message_passing_mpsc.rs:example}}
```

## Crossbeam_channel

[![crossbeam_channel][c-crossbeam_channel-badge]][c-crossbeam_channel]{{hi:crossbeam_channel}}  [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]{{hi:Concurrency}}

Multi-producer multi-consumer channels{{hi:Multi-consumer channels}} for Message passing.

```rust
{{#include ../../../deps/tests/cats/concurrency/message_passing_crossbeam_channel.rs:example}}
```

Example using specialized channels for tickers{{hi:Tickers}} and timeout{{hi:Timeouts}}

```rust
{{#include ../../../deps/tests/cats/concurrency/message_passing_crossbeam_channel_after_tick.rs:example}}
```

## See also

[![cat-concurrency][cat-concurrency-badge]][cat-concurrency]{{hi:Concurrency}}

[![crossbeam][c-crossbeam-badge]][c-crossbeam]{{hi:crossbeam}}

[![postage][c-postage-badge]][c-postage]{{hi:postage}}

[Message passing (rust book)][book-rust-message-passing]{{hi:Message passing}}⮳

[p-async-channels]: ../asynchronous/async_channels.md
{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO: organize see also
</div>
