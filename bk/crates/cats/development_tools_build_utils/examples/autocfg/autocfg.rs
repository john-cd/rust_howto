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

fn main() {
    let ac = autocfg::new();
    // Test for 128-bit integer support:
    ac.emit_has_type("i128");

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
