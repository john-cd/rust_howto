# Streams

Futures are about a single value that will eventually be produced, but many event sources naturally produce a stream of values over time.

```rust,editable,ignore,mdbook-runnable
{{#include ../../deps/examples/streams.rs}}
```

There are combinator-style methods such as `map`, `filter`, and `fold`, and their early-exit-on-error cousins `try_map`, `try_filter`, and `try_fold`.

To process multiple items from a stream concurrently, use the `for_each_concurrent` and `try_for_each_concurrent` methods:

```rust,editable,ignore,noplayground
{{#include ../../deps/examples/streams2.rs}}
```

See also [Tokio async-stream (GitHub)][async-stream-github]⮳ [![async-stream-badge]][async-stream].

{{#include ../refs/link-refs.md}}
