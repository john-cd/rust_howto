// ANCHOR: example
//! This example demonstrates how to use the `assert!` macro with a custom
//! failure message.

fn check_name(name: &str) {
    assert!(
        name.contains("Carol"),
        "Name did not contain 'Carol', value was `{}`",
        name
    );
}

fn main() {
    check_name("Carl");
}
// ANCHOR_END: example

#[should_panic]
#[test]
fn test() {
    main();
}
