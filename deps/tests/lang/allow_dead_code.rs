// ANCHOR: example
// Disables the `dead_code` lint
#[allow(dead_code)]
fn unused_function() {}

fn main() {}

// ANCHOR_END: example
#[test]
fn test() {
    main();
}
