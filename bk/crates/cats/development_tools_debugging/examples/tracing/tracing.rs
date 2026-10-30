// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates the basic usage of the `tracing` crate for logging
//! events.
//!
//! It showcases how to use different log levels (trace, debug, info, warn,
//! error) and how to add structured data to log messages.

use tracing::Level;
use tracing::debug;
use tracing::error;
use tracing::event;
use tracing::info;
use tracing::trace;
use tracing::warn;

/// Emits events at several levels and records spans.
fn main() {
    // Initialize a tracing subscriber to see the events below.
    let _ = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init();

    // Basic logging of a message at various severity levels.
    event!(Level::INFO, "something happened");
    error!("error!");
    warn!("warning!");
    info!("info!");
    debug!("debug info!");
    trace!("trace info!");

    // Set the level and modify the event's target.
    // A target is a string that categorizes part of the system where the span
    // or event occurred. The tracing macros default to using the module path
    // where the span or event originated as the target, but it may be
    // overridden.
    event!(target: "app_events", Level::TRACE, "something has happened!");

    // Structured fields on events (and spans) are specified using the syntax
    // `field_name = field_value`. Fields are separated by commas.
    // Here, we record an event with two fields:
    event!(
        Level::INFO,
        answer = 42,
        question = "life, the universe, and everything"
    );

    // Unlike other fields, `message`'s shorthand initialization is just
    // the string itself.
    debug!(excitement = "yay!", "hello!");

    // String formatting; shorthand for `user = user`:
    let user = "ferris";
    event!(Level::TRACE, "login: {user}");

    // Records an event with a `struct` field using the `Debug` format.
    // Note the `?`: `my_struct` will be recorded
    // using its `fmt::Debug` implementation.
    let my_struct = S;
    event!(Level::TRACE, greeting = ?my_struct);
}

/// A simple struct for demonstration purposes.
#[derive(Debug)]
struct S;
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the tracing example's main path succeeds.
    /// Runs the tracing example through its entry point.
    #[test]
    fn test_tracing_example_entry_point() {
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
    fn events_carry_their_messages_fields_and_levels() {
        let out = capture(main);
        assert!(
            out.contains("INFO") && out.contains("something happened"),
            "{out}"
        );
        assert!(out.contains("ERROR") && out.contains("error!"), "{out}");
        assert!(out.contains("answer=42"), "{out}");
        assert!(out.contains("excitement=\"yay!\""), "{out}");
        assert!(out.contains("login: ferris"), "{out}");
        assert!(out.contains("greeting=S"), "{out}");
        assert!(out.contains("app_events"), "custom target is shown: {out}");
    }
}
