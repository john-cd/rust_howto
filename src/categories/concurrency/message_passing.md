# Message passing

{{#include message_passing.incl.md}}

One increasingly popular approach to ensuring safe concurrency is *message passing*{{hi:Message passing}}, where threads communicate by sending each other messages{{hi:Messages}} containing data. The Rust standard library provides *channels*{{hi:Channels}} for message passing that are safe to use in concurrent contexts{{hi:Concurrent contexts}}.

Message passing in [`async`][book-rust-reference-async]{{hi:async}}⮳ programming is covered in a separate page: [async channels][p-async-channels]{{hi:Async channels}}.

## Multiple producers, single consumer {#mpsc}

[![std][c-std-badge]][c-std]{{hi:std}} [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]{{hi:Concurrency}}{{hi:MPSC}}

```rust,editable
{{#include ../../../deps/tests/cats/concurrency/message_passing_mpsc.rs:example}}
```

## `crossbeam-channel` {#crossbeam-channel}

[![crossbeam-channel-website][c-crossbeam_channel-website-badge]][c-crossbeam_channel-website] [![crossbeam-channel][c-crossbeam_channel-badge]][c-crossbeam_channel] [![crossbeam-channel-crates.io][c-crossbeam_channel-crates.io-badge]][c-crossbeam_channel-crates.io] [![crossbeam-channel-github][c-crossbeam_channel-github-badge]][c-crossbeam_channel-github] [![crossbeam-channel-lib.rs][c-crossbeam_channel-lib.rs-badge]][c-crossbeam_channel-lib.rs]{{hi:crossbeam-channel}}{{hi:Channel}}{{hi:Select}}{{hi:Mpmc}}{{hi:Golang}}{{hi:Message}} [![cat-algorithms][cat-algorithms-badge]][cat-algorithms]{{hi:Algorithms}} [![cat-concurrency][cat-concurrency-badge]][cat-concurrency]{{hi:Concurrency}} [![cat-data-structures][cat-data-structures-badge]][cat-data-structures]{{hi:Data structures}}

Multi-producer multi-consumer channels{{hi:Multi-consumer channels}} for message passing.

```rust,editable
{{#include ../../../deps/tests/cats/concurrency/message_passing_crossbeam_channel.rs:example}}
```

Example using specialized channels for `tickers`{{hi:Tickers}} and `timeout`{{hi:Timeouts}}

```rust,editable
{{#include ../../../deps/tests/cats/concurrency/message_passing_crossbeam_channel_after_tick.rs:example}}
```

## See also

[![cat-concurrency][cat-concurrency-badge]][cat-concurrency]{{hi:Concurrency}}

[![crossbeam][c-crossbeam-badge]][c-crossbeam]{{hi:crossbeam}}

[![postage][c-postage-badge]][c-postage]{{hi:postage}}

[Message passing (rust book)][book-rust-message-passing]{{hi:Message passing}}⮳

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO P1 polish; organize see also
</div>
