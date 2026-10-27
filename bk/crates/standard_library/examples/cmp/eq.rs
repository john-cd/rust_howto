#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Implementing equality with `PartialEq` and `Eq`.

// `#[derive(PartialEq, Eq)]` automatically implements equality traits.
// When derived on structs, two instances are equal if all fields are equal,
// and not equal if any fields are not equal.
// When derived on enums, two instances are equal if they are the same variant
// and all fields are equal.
/// A point whose equality is derived.
#[derive(Debug, PartialEq, Eq)]
struct Point {
    /// The x coordinate.
    x: i32,
    /// The y coordinate.
    y: i32,
}

/// Compares points for equality.
fn points() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1, y: 2 };
    let p3 = Point { x: 3, y: 4 };

    println!("p1 == p2: {}", p1 == p2); // true
    println!("p1 == p3: {}", p1 == p3); // false
}

/// A person who is identified by an ID.
#[derive(Debug)]
struct Person {
    /// Unique identifier; the only field compared.
    id: u64,
    /// Last name of the person.
    last_name: String,
}

// Custom implementation of `PartialEq` and `Eq`:
impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
        // `last_name` is ignored.
    }
}

// `Eq` has no methods.
impl Eq for Person {}

/// Compares people for equality.
fn people() {
    let p1 = Person {
        id: 1,
        last_name: "Smith".to_owned(),
    };
    let p2 = Person {
        id: 1,
        last_name: "Jones".to_owned(),
    };
    assert_eq!(p1, p2);
    assert_ne!(p1.last_name, p2.last_name);
}

/// Runs the examples.
fn main() {
    points();
    people();
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the eq example through its entry point.
    #[test]
    fn test_eq_example_entry_point() {
        main();
    }
}
