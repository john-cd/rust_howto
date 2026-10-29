//! Runs the operator overloading example.
//!
//! TODO Reviewed ??-??-2026.

// Limitation: implementing `Fn*` traits for custom types needs the unstable
// `fn_traits` feature, so the `callable` module stays disabled on the stable
// channel. #![feature(unboxed_closures, fn_traits)]
// mod callable;

mod ops;

/// Runs the ops example.
fn main() {
    ops::run();
}
