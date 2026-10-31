// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Documentation comments on a struct and its constructor.
//!
//! `rustdoc` builds HTML documentation from doc comments. Intra-doc links such
//! as [`Vec`] connect items and are checked by `rustdoc`, which warns about
//! links that break after a rename.

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
    println!("{} -> {john:?}", john.name);
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the rustdoc2 example through its entry point.
    #[test]
    fn test_rustdoc2_example_entry_point() {
        main();
    }

    #[test]
    fn new_builds_a_person_with_the_given_name() {
        let john = new("John");
        assert_eq!(john.name, "John");
        assert_eq!(format!("{john:?}"), "Person { name: \"John\" }");
        assert_eq!(new("").name, "");
    }
}
