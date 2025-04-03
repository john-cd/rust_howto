// ANCHOR: example
use tracing::Level;
use tracing::event;
use tracing::instrument;

/// This function will be instrumented with a span.
///
/// The span will be named `my_function` and will have a field `my_arg`
/// with the value of the argument passed to the function.
#[instrument]
fn my_function(my_arg: usize) {
    // This event will be recorded inside a span named `my_function`
    // with the field `my_arg`.
    event!(Level::INFO, "inside my_function!");
    // ...
}

/// This async function will be instrumented with a span.
///
/// The span will be named `my_async_function` and will have a level of `info`.
///
/// The span will be exited when the function yields and re-entered when it
/// resumes.
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
