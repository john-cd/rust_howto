// ANCHOR: example
// Extension trait allowing futures to be instrumented with a tracing span,
// via `instrument(...)`
use tracing::Instrument;
use tracing::debug_span;
use tracing::info;
use tracing::info_span;
use tracing_subscriber;

async fn my_async_function() {
    // Constructs a span at the info level:
    let span = info_span!("my_async_function");
    // The above is equivalent to:
    // span!(Level::INFO, "my_async_function");

    // Executes some synchronous code in the context of this span:
    let _some_value = span.in_scope(|| {
        info!("I'm in the span!");
        42
    });
    info!("I'm not in the span!");

    // The `await` keyword may yield, causing the runtime to switch to another
    // task. However, the span has already been exited.
    some_other_async_function().await;

    // When the execution resumes from the await point, renter the span if
    // needed:
    span.in_scope(|| {
        info!("I'm also in the span!");
    });

    // We can also instrument async code:
    async move {
        // If we yield here, the span will be exited,
        // and re-entered when we resume.
        some_other_async_function().await;
    }
    .instrument(span) // Instrument the async block with the span...
    .await; // ...and await it.

    // We can also instrument an async function call.
    let _some_value = some_other_async_function()
        .instrument(debug_span!("some_other_async_function"))
        .await;
}

async fn some_other_async_function() {
    info!("I'm in some_other_async_function!");
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    my_async_function().await;
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
// [tracing_span_in_scope: does it print? (P0)](https://github.com/john-cd/rust_howto/issues/158)
