// ANCHOR: example
use tracing::Level;
use tracing::span;

/// Demonstrates entering and exiting a span using `entered()` and `exit()`.
fn main() {
    let span = span!(Level::TRACE, "some span").entered();

    println!("Code here is within the span");

    // Optionally, explicitly exit the span, returning it.
    let span = span.exit();

    println!("Code here is no longer within the span");

    // Enter the span again.
    let _span = span.entered();
    println!("Code here is within the span");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
