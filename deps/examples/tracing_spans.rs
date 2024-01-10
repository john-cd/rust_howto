use tracing::span;
use tracing::Level;

fn main() {
    let span = span!(Level::TRACE, "my_span");
    // `enter` returns a RAII guard which, when dropped, exits the span.
    // This indicates that we are in the span for the current lexical scope.
    {
        let _guard = span.enter();
        // Any trace events that occur here will occur within the span.
    }
    // Dropping the guard exits the span.
}
