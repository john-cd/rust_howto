#![allow(dead_code)]
// ANCHOR: example
//! `autocfg` is a library for build scripts to automatically configure code
//! based on compiler support. Code snippets are dynamically tested to see if
//! the rustc will accept them, rather than hard-coding specific version
//! support.
//!
//! Add to your `Cargo.toml`:
//! [build-dependencies]
//! autocfg = "1"

#[tracing::instrument]
fn main() {
    // Initialize tracing subscriber
    let _ = tracing_subscriber::fmt::try_init();
    tracing::info!("Starting autocfg example");

    let ac = autocfg::new();
    // Test for 128-bit integer support:
    tracing::info!("Testing for i128 support...");
    ac.emit_has_type("i128");
    tracing::info!("Emitted has_type check for i128.");
    println!("autocfg: checked i128 support (see CARGO_ENCODED_RUSTFLAGS for results)");

    // If the type test succeeds, this will write a `cargo:rustc-cfg=has_i128``
    // line for Cargo, which translates to Rust arguments `--cfg has_i128`. Then
    // in the rest of your Rust code, you can add `#[cfg(has_i128)]` conditions
    // on code that should only be used when the compiler supports it.

    // (optional) You don't need to rerun for anything external.
    // autocfg::rerun_path("build.rs");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
