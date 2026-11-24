// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Sorts a vector of structs, by their derived order and by a chosen field.
//!
//! Structs can be sorted by their derived ordering (field by field, in
//! declaration order) or by a key you choose, such as one field. Choose a key
//! when the default order is not the one users expect.

/// A struct with `name` and `age` fields.
///
/// It derives `PartialOrd` and `Ord` for ordering comparisons,
/// and `PartialEq` and `Eq` for equality comparisons.
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Person {
    /// The person's name.
    name: String,

    /// The person's age in years.
    age: u32,
}

impl Person {
    /// Constructor for creating a new `Person` instance.
    fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }
}

/// Sorts people by derived order, descending age, and a custom comparison.
fn main() {
    let mut people = vec![
        Person::new("Zoe".to_owned(), 25),
        Person::new("Al".to_owned(), 60),
        Person::new("John".to_owned(), 1),
    ];

    // Derived `Ord` compares names first and breaks ties by age.
    people.sort();

    assert_eq!(
        people,
        vec![
            Person::new("Al".to_owned(), 60),
            Person::new("John".to_owned(), 1),
            Person::new("Zoe".to_owned(), 25),
        ]
    );
    println!("{people:?}");

    // Sort people by age descending (largest age first):
    people.sort_by_key(|p| std::cmp::Reverse(p.age));

    assert_eq!(
        people,
        vec![
            Person::new("Al".to_owned(), 60),
            Person::new("Zoe".to_owned(), 25),
            Person::new("John".to_owned(), 1),
        ]
    );

    // Compare name lengths in bytes, then put the older person first on ties.
    people.sort_by(|left, right| {
        left.name
            .len()
            .cmp(&right.name.len())
            .then_with(|| right.age.cmp(&left.age))
    });

    assert_eq!(
        people,
        vec![
            Person::new("Al".to_owned(), 60),
            Person::new("Zoe".to_owned(), 25),
            Person::new("John".to_owned(), 1),
        ]
    );

    println!("{people:?}");
}
// ANCHOR_END: example

/// Sorts a vector of structs, by their derived order and by a chosen field.
pub fn run() {
    main();
}

/// Tests behavior implemented by the surrounding module.
#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the sort struct example through its entry point.
    #[test]
    fn test_sort_struct_example_entry_point() {
        main();
    }
}
