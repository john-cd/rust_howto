use tracing::{debug_span, info_span, Instrument};

async fn my_async_function() {
    let span = info_span!("my_async_function");

    // instrument synchronous code within an async functiom
    let _some_value = span.in_scope(|| {
        // run some synchronous code inside the span...
        42
    });

    // This is okay! The span has already been exited before we reach
    // the await point.
    some_other_async_function().await;

    // instrument async code
    async move {
        // This is correct! If we yield here, the span will be exited,
        // and re-entered when we resume.
        some_other_async_function().await;
    }
    .instrument(span) // instrument the async block with the span...
    .await; // ...and await it.

    let _some_value = some_other_async_function()
        .instrument(debug_span!("some_other_async_function"))
        .await;
}

async fn some_other_async_function() {}

#[tokio::main]
async fn main() {
    my_async_function().await;
}
