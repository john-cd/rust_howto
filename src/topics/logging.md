# Logs

[Tokio tracing]( https://github.com/tokio-rs/tracing )

[tracing-subscriber]( https://crates.io/crates/tracing-subscriber )

Add to `Cargo.toml`

```toml
[dependencies]
tracing = "0.1"
tracing-subscriber = "0.3"
```

## Initialization

Basic tracing

```rust,ignore
use tracing_subscriber;

fn main() {
    tracing_subscriber::fmt::init();  // filter events at runtime using environment variables: RUST_LOG=debug,my_crate=trace
}
```

Combine layers

```rust,ignore
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| {
                "myproj=debug,axum=debug,tower_http=debug,mongodb=debug".into()
            }),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
}
```

Or with a custom formatting layer

```rust,ignore
use tracing_subscriber::{fmt, EnvFilter};
use tracing_subscriber::prelude::*;

fn main() {
    let fmt_layer = fmt::layer()
        .with_target(false);
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();
}
```

Configure a custom event formatter

```rust,ignore
use tracing_subscriber::fmt;

fn main() {
    // Configure a custom event formatter
    let format = fmt::format()
    .with_level(false) // don't include levels in formatted output
    .with_target(false) // don't include targets
    .with_thread_ids(true) // include the thread ID of the current thread
    .with_thread_names(true) // include the name of the current thread
    .compact(); // use the `Compact` formatting style.

    // Create a `fmt` subscriber that uses our custom event format, and set it as the default.
    tracing_subscriber::fmt()
        .event_format(format)
        .init();
}
```

## Events

```rust,ignore
use tracing::{event, error, warn, info, debug, trace, Level};

fn main() {
    event!(Level::INFO, "something happened");
    error!("!");
    warn!("!");
    info!("!");
    debug!("!");
    trace!("!");

    event!(target: "app_events", Level::TRACE, "something has happened!");

    // records an event with two fields (also works for spans)
    event!(Level::INFO, answer = 42, question = "life, the universe, and everything");

    // unlike other fields, `message`'s shorthand initialization is just the string itself.
    debug!(excitement = "yay!", "hello!");

    // shorthand for user = user
    let user = "ferris";
    event!(Level::TRACE, "login", user);

    // `my_struct` will be recorded using its `fmt::Debug` implementation.
    event!(Level::TRACE, greeting = ?my_struct);
}
```

## Spans

```rust,ignore
use tracing::{span, Level};

fn main() {
    let span = span!(Level::TRACE, "my_span");
    // `enter` returns a RAII guard which, when dropped, exits the span. this
    // indicates that we are in the span for the current lexical scope.
    {
        let _guard = span.enter();
        // Any trace events that occur before the guard is dropped will occur within the span.
    }   // Dropping the guard will exit the span.
}
```

One-liner with `.entered()`:

```rust,ignore
use tracing::{span, Level};

fn main() {
    let span = span!(Level::TRACE, "some span").entered();

    // code here is within the span

    // optionally, explicitly exit the span, returning it
    let span = span.exit();

    // code here is no longer within the span

    // enter the span again
    let span = span.entered();
}
```

Holding the drop guard returned by `Span::enter` across `.await` points will result in incorrect traces. Use `in_scope`

```rust,ignore
async fn my_async_function() {
    let span = info_span!("my_async_function");

    // instrument synchronous code within an async functiom
    let some_value = span.in_scope(|| {
        // run some synchronous code inside the span...
    });

    // This is okay! The span has already been exited before we reach
    // the await point.
    some_other_async_function(some_value).await;

    // instrument async code
    async move {
    // This is correct! If we yield here, the span will be exited,
    // and re-entered when we resume.
    some_other_async_function().await;
    }
    .instrument(span)     // instrument the async block with the span...
    .await                    // ...and await it.

    let some_value = some_other_async_function()
    .instrument(debug_span!("some_other_async_function"))
    .await;
}
```

## Add tracing spans to functions

```rust,ignore
use tracing::{Level, event, instrument};

#[instrument]
pub fn my_function(my_arg: usize) {

    // This event will be recorded inside a span named `my_function` with the
    // field `my_arg`.
    event!(Level::INFO, "inside my_function!");
    // ...
}

// used on an async function
#[tracing::instrument(level = "info")]
async fn my_async_function() {

    // This is correct! If we yield here, the span will be exited,
    // and re-entered when we resume.
    some_other_async_function().await;
}
```
