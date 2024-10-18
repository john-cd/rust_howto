#![allow(dead_code)]

/// Returns a person with the name given them
///
/// # Arguments
///
/// * `name` - A string slice that holds the name of the person
///
/// # Examples
///
/// ```
/// // You can have rust code between fences inside the comments
/// // If you pass --test to `rustdoc`, it will even test it for you!
/// use doc::Person;
/// let person = Person::new("name");
/// ```
fn new(name: &str) -> Person {
    Person {
        name: name.to_string(),
    }
}

struct Person {
    name: String,
}

#[test]
fn test() {
    let _ = new("John");
}
