use parking_lot::Once;

static START: Once = Once::new();

#[test]
fn test() {
    // run a one-time initialization
    START.call_once(|| {
        // run initialization here
    });
}
