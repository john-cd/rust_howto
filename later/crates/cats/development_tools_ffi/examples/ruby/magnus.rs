#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Calling Rust from Ruby with `magnus`.

use magnus::Error;
use magnus::value::ReprValue as _;

/// Simple function to be called from Ruby.
fn hello_rust() -> String {
    super::greeting::message()
}

/// Calls Ruby's `puts` with a string created in Rust.
fn main() -> Result<(), Error> {
    // SAFETY: this standalone entry point initializes the sole VM and keeps
    // the guard alive until all Ruby execution has finished.
    let ruby = unsafe { magnus::embed::init() };

    // Create a new Ruby string object from the given Rust string.
    let string = ruby.str_new(&hello_rust());

    // Safely call the `puts` method via the Object class without evaluating
    // dynamic strings.
    ruby.class_object()
        .funcall::<_, _, magnus::Value>("puts", (string,))?;

    // Define a global Ruby function that calls a Rust function.
    // define_global_function("hello_rust", hello_rust);

    // Evaluate Ruby code that calls the Rust function
    // eval::<magnus::Value>(
    // r#"
    // puts hello_rust()
    // "#,
    // )?;

    Ok(())
}
// ANCHOR_END: example

pub fn run() -> Result<(), Error> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the magnus example through its entry point.
    #[test]
    #[ignore = "needs Ruby; run this test alone in its own process"]
    fn test_magnus_example_entry_point() {
        main().expect("example entry point should succeed");
    }
}
// [finish](https://github.com/john-cd/rust_howto/issues/1035)
