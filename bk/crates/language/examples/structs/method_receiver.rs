//! Shows automatic borrowing of a method receiver.
//!
//! TODO Reviewed ??-??-2026.

struct Point(i32);

impl Point {
    /// Returns the distance between two points on a line.
    fn distance(&self, other: &Self) -> u32 {
        self.0.abs_diff(other.0)
    }
}

/// Method-call syntax borrows the receiver when the method takes a reference.
#[test]
#[allow(
    clippy::needless_borrow,
    reason = "Shows the implicit and explicit forms."
)]
fn explicit_and_implicit_borrows_match() {
    let p1 = Point(2);
    let p2 = Point(5);

    // ANCHOR: example
    let implicit = p1.distance(&p2);
    let explicit = (&p1).distance(&p2);
    // ANCHOR_END: example

    assert_eq!(implicit, 3);
    assert_eq!(implicit, explicit);
}
