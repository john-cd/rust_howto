//! Compile-fail fixture for an exclusive reborrow while a shared borrow remains active.
//!
//! TODO Reviewed ??-??-2026.

/// Demonstrates an exclusive reborrow conflicting with a live shared borrow.
fn main() {
    let mut value = 42;
    let exclusive = &mut value;
    let first = &*exclusive;
    let _mutable_reborrow = &mut *exclusive;
    println!("{first}");
}
