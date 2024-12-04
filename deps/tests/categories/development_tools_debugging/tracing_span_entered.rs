// ANCHOR: example
use tracing::Level;
use tracing::span;

fn main() {
    let span = span!(Level::TRACE, "some span").entered();

    println!("Code here is within the span");

    // optionally, explicitly exit the span, returning it
    let span = span.exit();

    println!("Code here is no longer within the span");

    // enter the span again
    let _span = span.entered();
    println!("Code here is within the span");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
