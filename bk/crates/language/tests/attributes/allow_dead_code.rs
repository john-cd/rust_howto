// ANCHOR: example
// Disables the `dead_code` lint for the entire file.
#![allow(dead_code)]

/// This function is defined but never called.
///
/// The attribute above disables the `dead_code` lint,
/// which would otherwise warn about this unused function.
///
/// You could also add a `#[allow(dead_code)]` attribute here
/// to disable the lint for this function only.
fn unused_function() {}

fn main() {
    println!("Nobody is calling `unused_function`.");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
