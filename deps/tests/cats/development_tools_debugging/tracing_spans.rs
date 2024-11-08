// ANCHOR: example
use tracing::span;
use tracing::Level;

fn main() {
    let span = span!(Level::TRACE, "my_span");
    {
        // current lexical scope.
        let _guard = span.enter(); // `enter` returns a RAII guard
                                   // which, when dropped, exits the
                                   // span.
                                   // Any trace events that occur here
                                   // will occur within the span.
    } // Dropping the guard exits the span.
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
