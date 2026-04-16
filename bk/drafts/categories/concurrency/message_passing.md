# Message Passing and Channels

{{#include message_passing.incl.md}}

One increasingly popular approach to ensuring safe [concurrency][p~concurrency] is *message passing*{{hi:Message passing}}, where threads communicate by sending each other messages{{hi:Messages}} containing data. The Rust standard library provides *channels*{{hi:Channels}} for message passing that are safe to use in concurrent contexts{{hi:Concurrent contexts}}.

Message passing in [`async`][book~rust-reference~async]↗{{hi:async}} programming is covered in a separate page: [async channels][p~async-channels]↗{{hi:Async channels}}.

## Multiple Producers, Single Consumer {#mpsc}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}}{{hi:MPSC}}

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/message_passing/message_passing_mpsc.rs:example}}
```

## `crossbeam-channel` {#crossbeam-channel}

[![crossbeam-channel~website][c~crossbeam-channel~website~badge]][c~crossbeam-channel~website] [![crossbeam-channel][c~crossbeam-channel~docs~badge]][c~crossbeam-channel~docs] [![crossbeam-channel~crates.io][c~crossbeam-channel~crates.io~badge]][c~crossbeam-channel~crates.io] [![crossbeam-channel~repo][c~crossbeam-channel~repo~badge]][c~crossbeam-channel~repo] [![crossbeam-channel~lib.rs][c~crossbeam-channel~lib.rs~badge]][c~crossbeam-channel~lib.rs]{{hi:crossbeam-channel}}{{hi:Channel}}{{hi:Select}}{{hi:Mpmc}}{{hi:Golang}}{{hi:Message}} [![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}

[`crossbeam-channel`][c~crossbeam-channel~docs]↗{{hi:crossbeam-channel}} offers multi-producer multi-consumer channels{{hi:Multi-consumer channels}} for message passing. The absolute fastest channel implementation available. Implements Go-like 'select' feature.

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/message_passing/message_passing_crossbeam_channel.rs:example}}
```

Example using specialized channels for [`tickers`][c~tickers~docs]↗{{hi:tickers}}{{hi:Tickers}} and [`timeout`][c~reqwest::ClientBuilder::timeout~docs]↗{{hi:Timeout}}

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/message_passing/message_passing_crossbeam_channel_after_tick.rs:example}}
```

### `flume` {#flume}

[![flume][c~flume~docs~badge]][c~flume~docs]{{hi:flume}}
[![flume~crates.io][c~flume~crates.io~badge]][c~flume~crates.io]
[![flume~repo][c~flume~repo~badge]][c~flume~repo]
[![flume~lib.rs][c~flume~lib.rs~badge]][c~flume~lib.rs]

The [`flume`][c~flume~docs]↗{{hi:flume}} crate is a library that provides multiple-producer, multiple-consumer (MPMC) channels. It is similar to the [`std::sync::mpsc`][c~std::sync::mpsc::channel~docs]↗{{hi:std::sync::mpsc}} module, but with additional features and improved performance. It is smaller and simpler than [`crossbeam-channel`][c~crossbeam-channel~docs]↗{{hi:crossbeam-channel}} and almost as fast.

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/message_passing/flume.rs:example}}
```

## See Also {#see-also .skip}

- [Message passing (rust book)][book~rust~message-passing]↗{{hi:Message passing}}.

## Related Topics {#related-topics .skip}

FIXME

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[message_passing: polish](https://github.com/john-cd/rust_howto/issues/264)
</div>
