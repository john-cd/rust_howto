// ANCHOR: example
use tracing::event;
use tracing::instrument;
use tracing::Level;

#[instrument]
fn my_function(my_arg: usize) {
    // This event will be recorded inside a span named `my_function`
    // with the field `my_arg`.
    event!(Level::INFO, "inside my_function!");
    // ...
}

// used on an async function
#[instrument(level = "info")]
async fn my_async_function() {
    // This is correct! If we yield here, the span will be exited,
    // and re-entered when we resume.
    some_other_async_function().await;
}

async fn some_other_async_function() {}

#[tokio::main]
async fn main() {
    my_function(42);
    my_async_function().await;
}

// ANCHOR_END: example
#[test]
fn test() {
    main();
}
