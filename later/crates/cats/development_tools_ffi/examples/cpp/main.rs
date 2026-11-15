//! Runs the C++ interoperability example.
//!
//! TODO Reviewed ??-??-2026.

/// Calling C++ from Rust with the `cxx` crate.
mod cxx;

/// Runs the cpp example.
fn main() {
    cxx::run();
}
