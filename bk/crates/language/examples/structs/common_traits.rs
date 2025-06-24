#![allow(dead_code)]
// ANCHOR: example
//! Implementation of Common Traits on a `struct`.

use std::fmt;
use std::hash::Hash;

/// The `derive` attribute allows the quick and implementation of very common
/// traits:
/// - The `Debug` trait is used to provide a programmer-facing, detailed, string
///   representation of a type.
/// - `Clone` allows you to create an explicit, independent copy (a "clone") of
///   a value.
/// - `Copy` is a special marker trait indicating that a type's values can be
///   duplicated by a simple bitwise copy (like copying bytes in memory). Types
///   that are Copy are implicitly copied when they are assigned, passed to
///   functions, or returned from functions, rather than being moved (which
///   would invalidate the original). All fields (i32) are `Copy`, so Point2D
///   can be `Copy`.
/// - `PartialEq` (and `Eq`) allow instances of a type to be compared for
///   equality (==) and inequality (!=).
/// - `PartialOrd` (and `Ord`) allows instances of a type to be ordered, meaning
///   you can compare them using <, >, <=, and >=.
/// - `Hash` allows instances of a type to be "hashed," which means producing a
///   single u64 value that represents the instance.
/// - `Default` allows for the creation of a "default" or "initial" instance of
///   a type, often representing a zeroed, empty, or baseline state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct Point2D {
    x: i32,
    y: i32,
}

// These traits can of course be manually implemented:
// impl fmt::Debug for Point2D {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("Point")
//             .field("x", &self.x)
//             .field("y", &self.y)
//             .finish()
//     }
// }

// The `Display` trait allows your struct to be printed using the `{}` format
// specifier, for user-friendly output.
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point2D: ({}, {})", self.x, self.y)
    }
}

fn main() {
    // Create an instance of the `Point2D` struct:
    let p = Point2D { x: 1, y: 1 };

    // Use the `Debug` implementation:
    println!("{:?}", p);

    // Use the `Display` implementation:
    println!("{}", p);

    // Clone the point.
    // Since the type is `Copy`, you can and should just assign.
    #[allow(clippy::clone_on_copy)]
    let p2 = p.clone();
    println!("{:?}", p2);

    // Since `Point2D` is `Copy`, p2 remains valid after an assignment.
    let p3 = p2;
    println!("{:?}", p2);
    println!("{:?}", p3);

    // Equality:
    assert_eq!(p2, p3);

    // Use the `Hash` implementation:
    let mut h = std::collections::hash_map::DefaultHasher::new();
    p.hash(&mut h);
    // `Point2D` can be used as a key in a `HashMap`.

    // Use the `Default` implementation:
    let p4 = Point2D::default();
    println!("{:?}", p4);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
