// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Generating bindings to other languages with `uniffi`.

// #![allow(non_snake_case)]

// This example demonstrates how to use UniFFI to create a Rust library that
// can be called from other languages.

// uniffi::setup_scaffolding!();

// #[uniffi::export]
/// Returns a greeting for `name`.
pub fn greet(name: String) -> String {
    format!("Hello, {name}!")
}

// #[uniffi::export] // marks as available for FFI.
/// Adds two numbers.
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Prints the sum of two numbers.
fn main() {
    println!("{}", add(2, 3));
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the uniffi example through its entry point.
    #[test]
    fn test_uniffi_example_entry_point() {
        main();
    }
}
// [finish; https://github.com/mozilla/uniffi-rs/blob/main/examples/arithmetic/Cargo.toml](https://github.com/john-cd/rust_howto/issues/1037)
