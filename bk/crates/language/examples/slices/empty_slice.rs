//! Shows how to take an empty slice.
//!
//! TODO Reviewed ??-??-2026.

/// An empty prefix contains no elements.
#[test]
fn empty_prefix() {
    // ANCHOR: empty
    let x = [1, 2, 3];
    let empty = &x[..0]; // Same as &x[0..0].
    assert_eq!(empty, &[]);
    // ANCHOR_END: empty
}
