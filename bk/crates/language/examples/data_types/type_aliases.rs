//! Shows aliases for scalar, tuple, and generic types.
//!
//! TODO Reviewed ??-??-2026.

// ANCHOR: aliases
/// Names a distance unit without introducing a distinct type or conversion.
type Kilometers = u32;

/// Alias for `(i32, i32)` used to represent point.
type Point = (i32, i32);

// A generic alias names an existing generic type.
/// Gives Vec<T> another name while retaining all of its original operations.
type TypeAlias<T> = Vec<T>;
// ANCHOR_END: aliases

/// Aliases accept values of their underlying types.
#[test]
fn aliases_accept_the_original_types() {
    let distance: Kilometers = 3_u32;
    let point: Point = (1, 2);
    let points: TypeAlias<Point> = vec![point];
    assert_eq!(distance, 3);
    assert_eq!(points, [(1, 2)]);
}
