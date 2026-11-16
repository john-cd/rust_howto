//! Shows reserving, borrowing, shrinking, and clearing a vector.
//!
//! TODO Reviewed ??-??-2026.

// ANCHOR: example
/// Creates and reuses a vector without copying its elements.
fn main() {
    let mut values = Vec::with_capacity(100);
    assert!(values.capacity() >= 100);
    values.extend([1, 2, 3]);

    // Reduce excess capacity when future growth is unlikely.
    values.shrink_to_fit();

    // Borrow a slice when the callee does not need ownership.
    assert_eq!(sum(&values), 6);

    // Clear the contents while keeping the allocation for reuse.
    let capacity = values.capacity();
    values.clear();
    assert_eq!(values.capacity(), capacity);
    assert!(values.is_empty());
}

/// Reads a slice of integers without owning it.
fn sum(values: &[i32]) -> i32 {
    values.iter().sum()
}
// ANCHOR_END: example

/// Reuses capacity after clearing the vector and reads elements through a slice.
#[test]
fn vector_capacity_can_be_reused() {
    main();
}
