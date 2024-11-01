// ANCHOR: example
use parking_lot::Once;

static START: Once = Once::new();

fn main() {
    // run a one-time initialization
    START.call_once(|| {
        // run initialization here
    });
}

// TODO add more

// ANCHOR_END: example
#[test]
fn test() {
    main();
}
