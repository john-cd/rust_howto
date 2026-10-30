// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Demonstrates entering and exiting a span using `entered()` and `exit()`.

use tracing::Level;
use tracing::span;

/// Demonstrates entering and exiting a span using `entered()` and `exit()`.
fn main() {
    // Initialize a tracing subscriber to see the span events below.
    let _ = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init();

    // Create a span, entering it using `entered()`, thus consuming it and
    // returning a guard that will exit the span when dropped.
    {
        let _span = span!(Level::INFO, "Some span").entered();

        tracing::info!("Code here is within the span.");
    }

    tracing::info!("Code here is no longer within the span.");

    // Optionally, use `entered` and `exit` to get in and out of the span.
    {
        let span = span!(Level::TRACE, "Another span").entered();
        tracing::info!("Within the span.");

        // Explicitly exit the span, returning it.
        let span = span.exit();
        tracing::info!("No longer within the span.");

        // Re-enter the span.
        let _span = span.entered();
        tracing::info!("Within the span.");
    }
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the tracing span entered example's main path succeeds.
    /// Runs the tracing span entered example through its entry point.
    #[test]
    fn test_tracing_span_entered_example_entry_point() {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::TRACE)
            .init();
        main();
    }

    #[derive(Clone, Default)]
    struct Buf(std::sync::Arc<std::sync::Mutex<Vec<u8>>>);

    impl std::io::Write for Buf {
        fn write(&mut self, data: &[u8]) -> std::io::Result<usize> {
            self.0.lock().unwrap().extend_from_slice(data);
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
        String::from_utf8(buf.0.lock().unwrap().clone()).unwrap()
    }

    #[test]
    fn events_are_inside_the_span_only_while_it_is_entered() {
        let out = capture(main);
        let line = |needle: &str| {
            out.lines()
                .find(|l| l.contains(needle))
                .unwrap_or_else(|| panic!("{needle} in {out}"))
        };
        assert!(line("Code here is within the span.").contains("Some span"));
        assert!(
            !line("Code here is no longer within the span.")
                .contains("Some span")
        );
        assert!(!line("No longer within the span.").contains("Another span"));
        assert!(
            out.lines()
                .filter(|l| l.contains("Within the span.")
                    && l.contains("Another span"))
                .count()
                == 2,
            "{out}"
        );
    }
}
