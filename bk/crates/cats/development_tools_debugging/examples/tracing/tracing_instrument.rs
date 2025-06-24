#![allow(dead_code)]
// ANCHOR: example
use tracing::Level;
use tracing::event;
use tracing::instrument;
use tracing::trace;

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
/// The span will be named `my_async_span`.
///
/// The span will be exited when the function yields and re-entered when it
/// resumes.
#[instrument(name = "my_async_span")]
async fn my_async_function() {
    trace!("I'm in my_async_span.");

    // If we yield here, the span will be exited,
    // and re-entered when we resume.
    some_other_async_function().await;
}

async fn some_other_async_function() {
    trace!("I'm in some_other_async_function.");
}

#[tokio::main]
async fn main() {
    my_function(42);
    my_async_function().await;
}
// ANCHOR_END: example

use rusty_fork::rusty_fork_test;

// Runs in a separate process.
rusty_fork_test! {
#[test]
    fn test() {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::TRACE)
            .init();
        main();
    }
}
