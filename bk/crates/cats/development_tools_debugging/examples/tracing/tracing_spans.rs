// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Creating and entering spans with `tracing`.

use tracing::Level;
use tracing::span;

/// Creates a span, enters it and emits events inside.
fn main() {
    // Create a new span named "my_span" with the TRACE level.
    // Spans are used to represent a period of time in the execution of a
    // program.
    let span = span!(Level::TRACE, "my_span");
    {
        // Current lexical scope.
        let _guard = span.enter();
        println!(
            "`enter` returns a RAII guard, which, when dropped, exits the span."
        );
        tracing::info!(
            "Any trace events that occur here will occur within the span."
        );
    }
    tracing::info!("Dropping the guard exits the span.");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that the span example runs.
    /// Runs the tracing spans example through its entry point.
    #[test]
    fn test_tracing_spans_example_entry_point() {
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
    fn events_are_inside_the_span_only_while_the_guard_lives() {
        let out = capture(main);
        let within = out
            .lines()
            .find(|l| l.contains("Any trace events"))
            .expect("captured output contains the span event");
        let after = out
            .lines()
            .find(|l| l.contains("Dropping the guard"))
            .expect("captured output contains the guard drop event");
        assert!(within.contains("my_span"), "{out}");
        assert!(!after.contains("my_span"), "{out}");
    }
}
