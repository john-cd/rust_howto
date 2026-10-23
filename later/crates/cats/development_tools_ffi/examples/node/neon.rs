#![allow(dead_code)]
#![allow(unexpected_cfgs)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! This example demonstrates how to create a simple Neon module that exports
//! a function to JavaScript.
//!
//! `neon` is a library for embedding Rust code in 'Node.js' applications.
//! It is slower than `napi`, but also widely used and well-maintained.
//!
//! # Steps
//! - Install the `neon` crate first: `cargo install neon`
//! - Create a new Neon project: `npm init neon my-neon-example`
//! - Build the native module: `cargo build`
//! - Run the JavaScript file: `node index.js`

#[cfg(feature = "neon")]
use neon::prelude::*;

/// A simple function that returns a JavaScript string.
#[cfg(feature = "neon")]
fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    // Create a JavaScript string
    Ok(cx.string("hello from neon!"))
}

// Entry point of the Neon module.
/// Registers the `hello` function in the exports of the Node.js module.
#[cfg(feature = "neon")]
#[::neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    // Expose the hello function to JavaScript
    cx.export_function("hello", hello)?;
    Ok(())
}
// ANCHOR_END: example

/// Prints a message when the `neon` feature is not enabled.
#[cfg(not(feature = "neon"))]
fn main() {
    println!("Neon example requires the `neon` feature.");
}

/// Runs the Neon example.
///
/// Note: This is a placeholder as Neon modules are typically executed
/// within a Node.js environment.
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    main();
    Ok(())
}

#[cfg(test)]
mod tests {
    // use super::*;

    /// Verifies that the neon example's main path succeeds.
    #[test]
    #[ignore = "requires Node module runtime"]
    fn test_main() {
        // TODO main().unwrap();
    }
}
// [finish how to test](https://github.com/john-cd/rust_howto/issues/1033)?
