# Streams

[![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]

Futures are about a single value that will eventually be produced, but many event sources naturally produce a {{hi:Stream}}[`Stream`][c-futures::stream::Stream] of values over time.

{{#include streams.incl.md}}

```rust,editable,mdbook-runnable
{{#include ../../../deps/tests/streams.rs}}
```

There are combinator-style methods such as {{hi:map}}[`map`][c-futures::prelude::stream::StreamExt::map]⮳, {{hi:filter}}[`filter`][c-futures::prelude::stream::StreamExt::filter]⮳, and {{hi:fold}}[`fold`][c-futures::prelude::stream::StreamExt::fold]⮳, and their early-exit-on-error cousins {{hi:try_filter}}[`try_filter`][c-futures::prelude::stream::TryStreamExt::try_filter]⮳, and {{hi:try_fold}}[`try_fold`][c-futures::prelude::stream::TryStreamExt::try_fold]⮳.

To process multiple items from a {{i:stream}} concurrently, use the {{hi:for_each_concurrent}}[`for_each_concurrent`][c-futures::prelude::stream::StreamExt::for_each_concurrent]⮳ and {{hi:try_for_each_concurrent}}[`try_for_each_concurrent`][c-futures::prelude::stream::TryStreamExt::try_for_each_concurrent]⮳ methods:

```rust,editable,noplayground,no_run
{{#include ../../../deps/tests/streams2.rs}}
```

## See also

See also Tokio `async_stream`{{hi:async_stream}}.

[![async_stream][c-async_stream-badge]][c-async_stream]  [![async_stream-github][c-async_stream-github-badge]][c-async_stream-github]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
