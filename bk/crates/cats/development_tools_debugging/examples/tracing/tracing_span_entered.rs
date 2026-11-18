// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Demonstrates entering and exiting a span using `entered()` and `exit()`.
//!
//! A span marks a period of work, such as handling one request. Events logged
//! while the span is entered carry its context. Holding the entered guard
//! across an `.await` is a mistake in async code; use `instrument` on the
//! future instead.

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

/// Demonstrates entering and exiting a span using `entered()` and `exit()`.
pub fn run() {
    main();
}

/// Verifies that the tracing span entered example's main path succeeds.
#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the tracing span entered example's main path succeeds.
    #[test]
    fn test_tracing_span_entered_example_entry_point() {
        let _ = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::TRACE)
            .try_init();
        main();
    }

    /// Cloneable writer sharing one locked byte buffer for formatted tracing assertions.
    #[derive(Clone, Default)]
    struct Buf(
        /// Shared output buffer used by cloned writers to capture formatted bytes safely.
        std::sync::Arc<std::sync::Mutex<Vec<u8>>>,
    );

    impl std::io::Write for Buf {
        /// Appends formatted tracing bytes to the shared capture buffer.
        ///
        /// # Arguments
        ///
        /// * `data` - Bytes produced by the formatter.
        ///
        /// # Returns
        ///
        /// The number of bytes appended.
        ///
        /// # Errors
        ///
        /// Returns an I/O error if the buffer lock is poisoned.
        fn write(&mut self, data: &[u8]) -> std::io::Result<usize> {
            self.0
                .lock()
                .map_err(|_| std::io::Error::other("tracing output mutex is poisoned"))?
                .extend_from_slice(data);
            Ok(data.len())
        }

        /// Flushes the current value.
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    impl<'a> tracing_subscriber::fmt::MakeWriter<'a> for Buf {
        /// Writer type returned by this factory.
        type Writer = Self;

        /// Assembles the writer from its component values so callers receive a ready-to-use result.
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

    /// Confirms that the events are inside the span only while it is entered.
    #[test]
    fn events_are_inside_the_span_only_while_it_is_entered() {
        let out = capture(main);
        let line = |needle: &str| {
            out.lines()
                .find(|l| l.contains(needle))
                .unwrap_or_else(|| panic!("{needle} in {out}"))
        };
        assert!(line("Code here is within the span.").contains("Some span"));
        assert!(!line("Code here is no longer within the span.").contains("Some span"));
        assert!(!line("No longer within the span.").contains("Another span"));
        assert!(
            out.lines()
                .filter(|l| l.contains("Within the span.") && l.contains("Another span"))
                .count()
                == 2,
            "{out}"
        );
    }
}
