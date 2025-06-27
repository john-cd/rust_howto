// ANCHOR: example
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
/// let person = Person::new("name");
/// ```
fn new(name: &str) -> Person {
    Person {
        name: name.to_string(),
    }
}

#[derive(Debug)]
struct Person {
    name: String,
}

fn main() {
    let john = new("John");
    println!("{john:?}");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
