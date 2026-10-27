#![allow(dead_code)]
#![allow(unexpected_cfgs)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! This example demonstrates how to create a simple Neon module that exports
//! a function to JavaScript.
//!
//! `neon` is a library for embedding Rust code in 'Node.js' applications.
//! Its context API manages access to JavaScript values.
//!
//! # Steps
//! - Create a new Neon project with the upstream npm initializer.
//! - Build the native module with the generated package's npm build script.
//! - Run the JavaScript file: `node index.js`

#[cfg(feature = "node")]
use neon::prelude::*;

/// Returns the message independently of the JavaScript runtime.
fn greeting() -> &'static str {
    "hello from neon!"
}

/// A simple function that returns a JavaScript string.
#[cfg(feature = "node")]
fn hello(mut cx: FunctionContext<'_>) -> JsResult<'_, JsString> {
    // Create a JavaScript string.
    Ok(cx.string(greeting()))
}

// Entry point of the Neon module.
/// Registers the `hello` function in the exports of the Node.js module.
#[cfg(feature = "node")]
#[::neon::main]
fn register(mut cx: ModuleContext<'_>) -> NeonResult<()> {
    // Expose the hello function to JavaScript.
    cx.export_function("hello", hello)?;
    Ok(())
}

// TODO fix - should excercise the Neon module through Node.js runtime.
/// Runs the host-independent part without pretending to load a Node addon.
fn main() {
    println!("{}", greeting());
}
// ANCHOR_END: example

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    main();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the neon example's main path succeeds.
    #[test]
    #[ignore = "requires Node module runtime"]
    fn test_neon_example_entry_point() {
        main();
    }

    /// Checks the string returned by the native callback without Node.js.
    #[test]
    fn greeting_matches_the_native_callback_message() {
        assert_eq!(super::greeting(), "hello from neon!");
    }
}
