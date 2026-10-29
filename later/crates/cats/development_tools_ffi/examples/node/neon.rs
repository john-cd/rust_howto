#![allow(unexpected_cfgs)]
// TODO Reviewed ??-??-2026.

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

/// Registers the `hello` function in the exports of the Node.js module.
#[cfg(feature = "node")]
#[::neon::main]
fn register(mut cx: ModuleContext<'_>) -> NeonResult<()> {
    // Expose the hello function to JavaScript.
    cx.export_function("hello", hello)?;
    Ok(())
}

/// Runs the host-independent example and reports success.
fn main() -> anyhow::Result<()> {
    println!("{}", greeting());
    Ok(())
}
// ANCHOR_END: example

/// Runs the Neon example without loading its Node.js module.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the host-independent entry point succeeds without Node.js.
    #[test]
    fn test_neon_example_entry_point() {
        let result = run();

        assert!(result.is_ok(), "the Neon example failed: {result:?}");
    }

    /// Checks the string returned by the native callback without Node.js.
    #[test]
    fn test_main_entry_point() {
        main().expect("the Neon example should run successfully");
    }

    #[test]
    fn greeting_matches_the_native_callback_message() {
        assert_eq!(super::greeting(), "hello from neon!");
    }
}
