#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Sorts a vector of structs, by their derived order and by a chosen field.

/// A struct with `name` and `age` fields.
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Person {
    /// The person's name.
    name: String,
    /// The person's age in years.
    age: u32,
}

impl Person {
    /// Constructor for creating a new `Person` instance.
    pub fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }
}

/// Sorts a vector of `Person`s in several ways and prints the results.
fn main() {
    let mut people = vec![
        Person::new("Zoe".to_owned(), 25),
        Person::new("Al".to_owned(), 60),
        Person::new("John".to_owned(), 1),
    ];

    // Sort people by derived natural order (in this case, name and age):
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

    // Use a comparator to sort by name length, then age descending:
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

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
