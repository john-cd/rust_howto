//! Runs the reference counting example.
//!
//! TODO Reviewed ??-??-2026.

/// `Rc` is a smart pointer that allows multiple owners of the same data.
mod rc;

/// Runs the reference counting example.
fn main() {
    rc::run();
}
