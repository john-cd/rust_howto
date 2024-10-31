use tracing::span;
use tracing::Level;

fn main() {
    let span = span!(Level::TRACE, "some span").entered();

    // code here is within the span

    // optionally, explicitly exit the span, returning it
    let span = span.exit();

    // code here is no longer within the span

    // enter the span again
    let _span = span.entered();
}

#[test]
fn test() {
    main();
}
