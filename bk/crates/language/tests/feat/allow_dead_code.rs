// ANCHOR: example
// Disables the `dead_code` lint
#![allow(dead_code)]

/// This function is defined but never called.
///
/// The `#[allow(dead_code)]` attribute disables the `dead_code` lint,
/// which would otherwise warn about this unused function.
fn unused_function() {}

fn main() {
    println!("Nobody is calling `unused_function`.");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
