// ANCHOR: example
use tracing::span;
use tracing::Level;

fn main() {
    let span = span!(Level::TRACE, "my_span");
    {
        // Current lexical scope.
        let _guard = span.enter();
        println!("`enter` returns a RAII guard, which, when dropped, exits the span.");
        println!("Any trace events that occur here will occur within the span.")
    }
    println!("Dropping the guard exits the span.");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
