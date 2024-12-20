# Message passing and channels

{{#include message_passing.incl.md}}

One increasingly popular approach to ensuring safe concurrency is *message passing*{{hi:Message passing}}, where threads communicate by sending each other messages{{hi:Messages}} containing data. The Rust standard library provides *channels*{{hi:Channels}} for message passing that are safe to use in concurrent contexts{{hi:Concurrent contexts}}.

Message passing in [`async`][book-rust-reference-async]{{hi:async}}⮳ programming is covered in a separate page: [async channels][p-async-channels]{{hi:Async channels}}.

## Multiple producers, single consumer {#mpsc}

[![std][c-std-badge]][c-std]{{hi:std}} [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]{{hi:Concurrency}}{{hi:MPSC}}

```rust,editable
{{#include ../../../crates/ex/categories/c/tests/concurrency/message_passing_mpsc.rs:example}}
```

## `crossbeam-channel` {#crossbeam-channel}

[![crossbeam-channel-website][c-crossbeam_channel-website-badge]][c-crossbeam_channel-website] [![crossbeam-channel][c-crossbeam_channel-badge]][c-crossbeam_channel] [![crossbeam-channel-crates.io][c-crossbeam_channel-crates.io-badge]][c-crossbeam_channel-crates.io] [![crossbeam-channel-github][c-crossbeam_channel-github-badge]][c-crossbeam_channel-github] [![crossbeam-channel-lib.rs][c-crossbeam_channel-lib.rs-badge]][c-crossbeam_channel-lib.rs]{{hi:crossbeam-channel}}{{hi:Channel}}{{hi:Select}}{{hi:Mpmc}}{{hi:Golang}}{{hi:Message}} [![cat-algorithms][cat-algorithms-badge]][cat-algorithms]{{hi:Algorithms}} [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]{{hi:Concurrency}} [![cat-data-structures][cat-data-structures-badge]][cat-data-structures]{{hi:Data structures}}

Multi-producer multi-consumer channels{{hi:Multi-consumer channels}} for message passing. The absolute fastest channel implementation available. Implements Go-like 'select' feature.

```rust,editable
{{#include ../../../crates/ex/categories/c/tests/concurrency/message_passing_crossbeam_channel.rs:example}}
```

Example using specialized channels for `tickers`{{hi:Tickers}} and `timeout`{{hi:Timeouts}}

```rust,editable
{{#include ../../../crates/ex/categories/c/tests/concurrency/message_passing_crossbeam_channel_after_tick.rs:example}}
```

### `flume` {#flume}

[![flume][c-flume-badge]][c-flume]{{hi:flume}}
[![flume-crates.io][c-flume-crates.io-badge]][c-flume-crates.io]
[![flume-github][c-flume-github-badge]][c-flume-github]
[![flume-lib.rs][c-flume-lib.rs-badge]][c-flume-lib.rs]

Smaller and simpler than `crossbeam-channel` and almost as fast.

```rust,editable
{{#include ../../../crates/ex/categories/c/tests/concurrency/flume.rs:example}}
```

### `tokio` {#tokio}

[![tokio-website][c-tokio-website-badge]][c-tokio-website] [![tokio][c-tokio-badge]][c-tokio] [![tokio-crates.io][c-tokio-crates.io-badge]][c-tokio-crates.io] [![tokio-github][c-tokio-github-badge]][c-tokio-github] [![tokio-lib.rs][c-tokio-lib.rs-badge]][c-tokio-lib.rs]{{hi:tokio}}{{hi:Io}}{{hi:Async}}{{hi:Non-blocking}}{{hi:Futures}} [![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]{{hi:Asynchronous}} [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]{{hi:Network programming}}

Tokio's `sync` module provides channels for using in async code.

```rust,editable
{{#include ../../../crates/ex/categories/c/tests/concurrency/tokio.rs:example}}
```

## See also

[Message passing (rust book)][book-rust-message-passing]{{hi:Message passing}}⮳

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[message_passing: polish (P1)](https://github.com/john-cd/rust_howto/issues/264)
</div>
