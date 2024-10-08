# Streams

[![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]

Futures are about a single value that will eventually be produced, but many event sources naturally produce a [`{{i:Stream}}`][futures::stream::Stream] of values over time.

{{#include streams.incl.md}}

```rust,editable,mdbook-runnable
{{#include ../../../deps/tests/streams.rs}}
```

There are combinator-style methods such as [`map`][futures::prelude::stream::StreamExt::map]⮳, [`filter`][futures::prelude::stream::StreamExt::filter]⮳, and [`fold`][futures::prelude::stream::StreamExt::fold]⮳, and their early-exit-on-error cousins [`try_filter`][futures::prelude::stream::TryStreamExt::try_filter]⮳, and [`try_fold`][futures::prelude::stream::TryStreamExt::try_fold]⮳.

To process multiple items from a stream concurrently, use the [`for_each_concurrent`][futures::prelude::stream::StreamExt::for_each_concurrent]⮳ and [`try_for_each_concurrent`][futures::prelude::stream::TryStreamExt::try_for_each_concurrent]⮳ methods:

```rust,editable,noplayground,no_run
{{#include ../../../deps/tests/streams2.rs}}
```

## See also

See also Tokio `{{i:async-stream}}`.

[![async-stream][async-stream-badge]][async-stream]  [![async-stream-github][async-stream-github-badge]][async-stream-github]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
