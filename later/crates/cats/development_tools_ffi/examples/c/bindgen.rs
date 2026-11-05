// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Generate Rust declarations for a C library with bindgen.
//!
//! The package build script compiles example.c, runs bindgen on example.h, and
//! writes the generated declarations to Cargo's OUT_DIR.

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/// Calls the function bound with bindgen.
fn main() {
    // The generated declaration is an extern C function.
    let result = unsafe { add(20, 22) };
    assert_eq!(result, 42, "the C function returned {result}");
    println!("20 + 22 = {result}");
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Calls the function generated from the C header.
    /// Runs the bindgen example through its entry point.
    #[test]
    fn test_bindgen_example_entry_point() {
        main();
    }
}
