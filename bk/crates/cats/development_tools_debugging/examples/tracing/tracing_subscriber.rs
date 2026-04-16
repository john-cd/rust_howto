#![allow(dead_code)]
// ANCHOR: example
//! - `tracing` is a framework for instrumenting Rust programs to collect
//!   scoped, structured, and async-aware diagnostics.
//! - Its `Subscriber` trait represents the functionality necessary to collect
//!   this trace data.
//! - The `tracing-subscriber` crate contains tools for composing subscribers
//!   out of smaller units of behaviour, and batteries-included implementations
//!   of common subscriber functionality.
//! - In particular, the `tracing_subscriber::fmt` module provides a default
//!   implementation of the `Subscriber` trait that records `Events` and `Spans`
//!   by formatting them as text and logging them to stdout.
//!   [(docs.rs)](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/index.html)
//!
//! Add to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! tracing = "0.1.41" # Or latest version
//! tracing-subscriber = "0.3.20"
//! ```

fn main() {
    // In most cases, use the following to
    // initialize and install the global subscriber for your application:
    tracing_subscriber::fmt::init();

    // This is typicaly called early in the `main()` function of your
    // application. `init()` should not be called by libraries!
    // That will cause conflicts when executables try to set them later.

    // `fmt::init()` sets up a subscriber that logs to the console
    // and filters logs at runtime based on the `RUST_LOG` environment
    // variable, for example, `RUST_LOG=debug,my_crate=trace`.
    //
    // If the environment variable is empty or not set, or if it contains only
    // invalid directives, the ERROR level is used.

    // Test it out.
    tracing::error!("tracing configured!");
}
// `tracing_subscriber::fmt::init()` is equivalent to:
// tracing_subscriber::fmt() // Returns `SubscriberBuilder::default()`.
//     .with_env_filter(EnvFilter::from_default_env()) // Reads RUST_LOG.
//     .init(); // Installs the global default subscriber.
// ANCHOR_END: example

use rusty_fork::rusty_fork_test;

// Runs in a separate process.
rusty_fork_test! {
    #[test]
    fn test() {
        main();
    }
}
