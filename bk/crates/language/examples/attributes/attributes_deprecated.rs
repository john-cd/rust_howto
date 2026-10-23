#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Marking an item as deprecated with `#[deprecated]`.

#![allow(deprecated)]

// Mark a function as deprecated.
// That also works for structs, enums, etc...
/// A deprecated function.
#[deprecated(since = "5.2.0", note = "Use `bar` instead")]
pub fn foo() {
    println!("foo");
}

/// Calls a deprecated function.
fn main() {
    // Use of a deprecated item.
    // Normally, we would get a warning about using a deprecated function.
    foo();
    // In this case, the module-wide `#![allow(deprecated)]` attribute
    // (first line of the example) suppresses the warning.
}

// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
