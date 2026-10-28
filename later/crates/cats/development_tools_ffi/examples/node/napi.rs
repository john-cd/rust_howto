// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Calling Rust from Node.js with `napi`.

// Framework for building pre-compiled Node.js addons in Rust.

// Import the preludes:
// use napi_derive::napi;

// In the generated addon library, annotate the function with #[napi].
/// Returns a greeting for `name`.
fn hello(name: String) -> String {
    format!("Hello, {name}!")
}

/// Exercises the pure function before it is wrapped in a generated addon.
fn main() {
    println!("{}", hello("Node.js".to_owned()));
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the napi example through its entry point.
    #[test]
    fn test_napi_example_entry_point() {
        main();
    }

    /// Verifies the same greeting that the JavaScript smoke test expects.
    #[test]
    fn greeting_includes_the_javascript_argument() {
        assert_eq!(hello("Ada".to_owned()), "Hello, Ada!");
    }
}
// [finish; https://lib.rs/crates/napi; https://github.com/napi-rs/package-template](https://github.com/john-cd/rust_howto/issues/1032)
