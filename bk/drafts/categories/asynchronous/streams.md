<!-- TODO Reviewed ??-??-2026 -->

# Streams {#streams}

{{#include streams.incl.md}}

[![futures][c~futures~docs~badge]][c~futures~docs]{{hi:futures}} [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}}

An asynchronous computation produces a single value; a [`futures::stream::Stream`][c~futures::stream::Stream~docs]↗{{hi:futures::stream::Stream}} produces zero or more values over time. Stream extension traits provide adapters such as `map`, `filter`, and `fold`; `TryStreamExt` adds adapters that stop when an item is an error.

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/streams/streams.rs:example}}
```

There are combinator-style methods such as [`futures::prelude::stream::StreamExt::map`][c~futures::prelude::stream::StreamExt::map~docs]↗{{hi:futures::prelude::stream::StreamExt::map}}, [`futures::prelude::stream::StreamExt::filter`][c~futures::prelude::stream::StreamExt::filter~docs]↗{{hi:futures::prelude::stream::StreamExt::filter}}, and [`futures::prelude::stream::StreamExt::fold`][c~futures::prelude::stream::StreamExt::fold~docs]↗{{hi:futures::prelude::stream::StreamExt::fold}}, and their early-exit-on-error cousins [`futures::prelude::stream::TryStreamExt::try_filter`][c~futures::prelude::stream::TryStreamExt::try_filter~docs]↗{{hi:futures::prelude::stream::TryStreamExt::try_filter}}, and [`futures::prelude::stream::TryStreamExt::try_fold`][c~futures::prelude::stream::TryStreamExt::try_fold~docs]↗{{hi:futures::prelude::stream::TryStreamExt::try_fold}}.

To process multiple items concurrently, use [`StreamExt::for_each_concurrent`][c~futures::prelude::stream::StreamExt::for_each_concurrent~docs]↗{{hi:futures::prelude::stream::StreamExt::for_each_concurrent}} or [`TryStreamExt::try_for_each_concurrent`][c~futures::prelude::stream::TryStreamExt::try_for_each_concurrent~docs]↗{{hi:futures::prelude::stream::TryStreamExt::try_for_each_concurrent}}. The examples below simulate asynchronous work locally, so their tests do not require a network connection. A concurrency limit bounds the number of in-flight futures; these combinators do not create threads or make CPU-bound work parallel.

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/streams/streams2.rs:example}}
```

The following example showcases various utilities for working with streams:

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/streams/streams3.rs:example}}
```

## See Also {#see-also .skip}

For runtime integration, see [Tokio][p~tokio]. The [`async-stream`][c~async-stream~docs]↗{{hi:async-stream}} crate provides a macro for writing streams with asynchronous generator-style syntax.

[![async-stream][c~async-stream~docs~badge]][c~async-stream~docs]{{hi:async-stream}} [![async-stream~repo][c~async-stream~repo~badge]][c~async-stream~repo]

## Related Topics {#related-topics .skip}

- [[async | Asynchronous Programming]] introduces futures and runtimes.
- [[async_channels | Asynchronous Channels]] covers message passing between asynchronous tasks.
- [[structured_concurrency | Structured Concurrency]] covers task lifetimes and cancellation.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
