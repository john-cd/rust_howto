//! Models the required method of the iterator trait.
//!
//! TODO Reviewed ??-??-2026.

// ANCHOR: signature
/// Minimal iterator contract associating each implementation with its yielded type.
trait Iterator {
    /// Type of the item values produced or consumed by this trait.
    type Item;

    /// Returns the next item, or None when the iterator runs out.
    fn next(&mut self) -> Option<Self::Item>;
}
// ANCHOR_END: signature

/// Iterator that owns at most one value and consumes it on the first next call.
struct Once<T>(
    /// Value still available to yield; taking it leaves None and ends the iterator.
    Option<T>,
);

impl<T> Iterator for Once<T> {
    /// Alias for `T` used to represent item.
    type Item = T;

    /// Returns the next value produced by this iterator or protocol stream.
    fn next(&mut self) -> Option<Self::Item> {
        self.0.take()
    }
}

/// Successive calls advance the iterator to exhaustion.
#[test]
fn advances_to_exhaustion() {
    let mut once = Once(Some(7));
    assert_eq!(once.next(), Some(7));
    assert_eq!(once.next(), None);
}
