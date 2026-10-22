// TODO Reviewed ??-??-2026
// ANCHOR: example
//! Documentation comments on a struct and its constructor.

#![allow(dead_code)]

/// Returns a `Person` struct with the given name.
///
/// # Arguments
///
/// * `name` - A string slice that holds the name of the person.
///
/// # Examples
///
/// ```
/// // You can have Rust code between fences inside the comments.
/// // If you pass `--test` to `rustdoc`, it will even test it for you!
/// // `cargo test` runs doctest automatically as well.
/// let person = Person::new("name");
/// ```
fn new(name: &str) -> Person {
    Person {
        name: name.to_owned(),
    }
}

/// A person with a name.
#[derive(Debug)]
struct Person {
    /// Name of the person.
    name: String,
}

/// Creates a person and prints it.
fn main() {
    let john = new("John");
    println!("{john:?}");
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
