//! Models the required method of the iterator trait.
//!
//! TODO Reviewed ??-??-2026.

// ANCHOR: signature
trait Iterator {
    /// Type of the item values produced or consumed by this trait.
    type Item;

    /// Returns the next item, or None when the iterator runs out.
    fn next(&mut self) -> Option<Self::Item>;
}
// ANCHOR_END: signature

struct Once<T>(Option<T>);

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
