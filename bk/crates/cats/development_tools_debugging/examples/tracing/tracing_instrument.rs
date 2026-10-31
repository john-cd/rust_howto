// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Instrumenting sync and async functions with `tracing` spans.
//!
//! `tracing` records structured events and spans (timed regions of work),
//! unlike plain text logs. The `#[instrument]` attribute wraps a function in a
//! span automatically and records its arguments as fields.

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

/// Emits an event from inside the caller's span.
async fn some_other_async_function() {
    trace!("I'm in some_other_async_function.");
}

/// Initializes a subscriber and calls the instrumented functions.
#[tokio::main]
async fn main() {
    // Initialize a tracing subscriber to see the events below.
    let _ = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init();

    my_function(42);
    my_async_function().await;
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that the instrumented functions run.
    /// Runs the tracing instrument example through its entry point.
    #[test]
    fn test_tracing_instrument_example_entry_point() {
        let _ = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::TRACE)
            .try_init();
        main();
    }

    #[derive(Clone, Default)]
    struct Buf(std::sync::Arc<std::sync::Mutex<Vec<u8>>>);

    impl std::io::Write for Buf {
        fn write(&mut self, data: &[u8]) -> std::io::Result<usize> {
            self.0
                .lock()
                .map_err(|_| {
                    std::io::Error::other("tracing output mutex is poisoned")
                })?
                .extend_from_slice(data);
            Ok(data.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    impl<'a> tracing_subscriber::fmt::MakeWriter<'a> for Buf {
        type Writer = Self;

        fn make_writer(&'a self) -> Self {
            self.clone()
        }
    }

    /// Runs `f` with a thread-local subscriber and returns what it logged.
    fn capture(f: impl FnOnce()) -> String {
        let buf = Buf::default();
        let subscriber = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::TRACE)
            .with_writer(buf.clone())
            .with_ansi(false)
            .finish();
        tracing::subscriber::with_default(subscriber, f);
        let output = buf
            .0
            .lock()
            .expect("tracing output mutex should not be poisoned")
            .clone();
        String::from_utf8(output).expect("tracing output should be valid UTF-8")
    }

    #[test]
    fn instrumented_function_records_its_argument_in_a_span() {
        let out = capture(|| my_function(42));
        assert!(out.contains("my_function"), "{out}");
        assert!(out.contains("my_arg=42"), "{out}");
        assert!(out.contains("inside my_function!"), "{out}");
    }

    #[test]
    fn async_function_uses_the_custom_span_name() {
        let out = capture(|| {
            tokio::runtime::Builder::new_current_thread()
                .build()
                .expect("current-thread Tokio runtime should build")
                .block_on(my_async_function());
        });
        assert!(out.contains("my_async_span"), "{out}");
        assert!(out.contains("I'm in my_async_span."), "{out}");
        assert!(out.contains("I'm in some_other_async_function."), "{out}");
    }
}
