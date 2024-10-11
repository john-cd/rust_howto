# Streams

[![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]

Futures are about a single value that will eventually be produced, but many event sources naturally produce a [`{{i:Stream}}`][c-futures::stream::Stream] of values over time.

{{#include streams.incl.md}}

```rust,editable,mdbook-runnable
{{#include ../../../deps/tests/streams.rs}}
```

There are combinator-style methods such as [`{{i:map}}`][c-futures::prelude::stream::StreamExt::map]⮳, [`{{i:filter}}`][c-futures::prelude::stream::StreamExt::filter]⮳, and [`{{i:fold}}`][c-futures::prelude::stream::StreamExt::fold]⮳, and their early-exit-on-error cousins [`{{i:try_filter}}`][c-futures::prelude::stream::TryStreamExt::try_filter]⮳, and [`{{i:try_fold}}`][c-futures::prelude::stream::TryStreamExt::try_fold]⮳.

To process multiple items from a {{i:stream}} concurrently, use the [`{{i:for_each_concurrent}}`][c-futures::prelude::stream::StreamExt::for_each_concurrent]⮳ and [`{{i:try_for_each_concurrent}}`][c-futures::prelude::stream::TryStreamExt::try_for_each_concurrent]⮳ methods:

```rust,editable,noplayground,no_run
{{#include ../../../deps/tests/streams2.rs}}
```

## See also

See also Tokio `{{i:async_stream}}`.

[![async_stream][c-async_stream-badge]][c-async_stream]  [![async_stream-github][c-async_stream-github-badge]][c-async_stream-github]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
