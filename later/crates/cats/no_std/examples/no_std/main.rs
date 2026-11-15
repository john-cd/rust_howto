//! Runs the no std example.
//!
//! TODO Reviewed ??-??-2026.

/// A host binary that calls the library's core-only checksum implementation.
mod checksum;
/// Runs the checksum demonstration on the host.
fn main() {
    checksum::run();
}
