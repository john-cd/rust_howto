//! Mutates a value through a mutable reference.
//!
//! TODO Reviewed ??-??-2026.

/// Dereferencing the reference changes the original value.
#[test]
fn dereferencing_updates_the_original() {
    // ANCHOR: example
    let mut a: i32 = 1;

    // Create a mutable reference.
    let b: &mut i32 = &mut a;

    // Dereference it to change the original value.
    *b += 1;
    // ANCHOR_END: example
    assert_eq!(a, 2);
}
