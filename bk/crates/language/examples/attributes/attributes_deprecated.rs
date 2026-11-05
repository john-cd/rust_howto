//! TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Marking an item as deprecated with `#[deprecated]`.
//!
//! The `#[deprecated]` attribute warns callers that an API should no longer be
//! used. A version and a note can explain when it was deprecated and suggest a
//! replacement.

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
    // Call the deprecated function.
    foo();
    // Expected output: foo
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

    /// Runs the attributes deprecated example through its entry point.
    #[test]
    fn test_attributes_deprecated_example_entry_point() {
        main();
    }

    /// The deprecated function still works.
    #[test]
    fn test_deprecated_function_is_callable() {
        // Call the deprecated function.
        foo();
        // Expected output: foo
    }

    // Output-only example: `foo` only prints.
}
