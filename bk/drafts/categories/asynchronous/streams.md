# Streams {#streams}

{{#include streams.incl.md}}

[![futures][c~futures~docs~badge]][c~futures~docs]{{hi:futures}} [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}}

Futures are about a single value that will eventually be produced, but many event sources naturally produce a [`futures::stream::Stream`][c~futures::stream::Stream~docs]↗{{hi:futures::stream::Stream}} of values over time.

```rust,editable
{{#include ../../../crates/cats/asynchronous/examples/streams/streams.rs:example}}
```

There are combinator-style methods such as [`futures::prelude::stream::StreamExt::map`][c~futures::prelude::stream::StreamExt::map~docs]↗{{hi:futures::prelude::stream::StreamExt::map}}, [`futures::prelude::stream::StreamExt::filter`][c~futures::prelude::stream::StreamExt::filter~docs]↗{{hi:futures::prelude::stream::StreamExt::filter}}, and [`futures::prelude::stream::StreamExt::fold`][c~futures::prelude::stream::StreamExt::fold~docs]↗{{hi:futures::prelude::stream::StreamExt::fold}}, and their early-exit-on-error cousins [`futures::prelude::stream::TryStreamExt::try_filter`][c~futures::prelude::stream::TryStreamExt::try_filter~docs]↗{{hi:futures::prelude::stream::TryStreamExt::try_filter}}, and [`futures::prelude::stream::TryStreamExt::try_fold`][c~futures::prelude::stream::TryStreamExt::try_fold~docs]↗{{hi:futures::prelude::stream::TryStreamExt::try_fold}}.

To process multiple items from a stream{{hi:Stream}} concurrently, use the [`futures::prelude::stream::StreamExt::for_each_concurrent`][c~futures::prelude::stream::StreamExt::for_each_concurrent~docs]↗{{hi:futures::prelude::stream::StreamExt::for_each_concurrent}} and [`futures::prelude::stream::TryStreamExt::try_for_each_concurrent`][c~futures::prelude::stream::TryStreamExt::try_for_each_concurrent~docs]↗{{hi:futures::prelude::stream::TryStreamExt::try_for_each_concurrent}} methods:

```rust,editable,noplayground
{{#include ../../../crates/cats/asynchronous/examples/streams/streams2.rs:example}}
```

The following example showcases various utilities for working with streams:

```rust,editable,noplayground
{{#include ../../../crates/cats/asynchronous/examples/streams/streams3.rs:example}}
```

## See Also

See also [Tokio][p~tokio] [`async-stream`][c~async-stream~docs]↗{{hi:async-stream}}.

[![async-stream][c~async-stream~docs~badge]][c~async-stream~docs]{{hi:async-stream}} [![async-stream~github][c~async-stream~github~badge]][c~async-stream~github]

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[add more. streams2.rs is noplayground because it requires a network. rewrite](https://github.com/john-cd/rust_howto/issues/645)

- [AsyncStream][c~async-stream~lib.rs]↗.

</div>
