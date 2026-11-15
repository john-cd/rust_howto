//! Runs the `accesskit` accessibility example.
//!
//! Reviewed 10-27-2026.
//!
//! TODO Reviewed ??-??-2026.

/// Describes a small interface to screen readers with AccessKit.
mod accesskit;

/// Runs the screen readers example.
fn main() {
    accesskit::run();
}
