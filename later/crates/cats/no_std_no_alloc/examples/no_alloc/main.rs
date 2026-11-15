//! Runs the no alloc example.
//!
//! TODO Reviewed ??-??-2026.

/// A no-allocation example using fixed-capacity collections.
mod no_alloc;

/// Runs the no alloc example.
///
/// # Errors
///
/// Propagates fixed-capacity string formatting failures.
fn main() -> core::fmt::Result {
    no_alloc::run()
}
