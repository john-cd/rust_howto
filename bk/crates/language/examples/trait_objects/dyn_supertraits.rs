#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Trait objects of a trait with a supertrait.

// Supertrait.
/// A thing that can format a message.
trait Message {
    /// Returns `msg` decorated by the implementor.
    fn message(&self, msg: &str) -> String;
}

// Subtrait.
/// A thing that greets; requires [`Message`].
trait Greet: Message {
    /// Returns a greeting.
    fn greet(&self) -> String {
        self.message("Hello.")
    }
}

// An example struct that implements `Greet`:
/// A person.
struct Person {
    /// Name of the person.
    name: String,
}

impl Greet for Person {}

// The struct must also implement the supertrait:
impl Message for Person {
    fn message(&self, msg: &str) -> String {
        format!("{}: {msg}", self.name)
    }
}

// This function takes a trait object.
/// Greets and says goodbye through a trait object.
fn say_hello_and_goodbye(entity: &dyn Greet) {
    println!("{}", entity.greet());

    // Because `Greet` has `Message` as a supertrait,
    // we can also call `Message`'s methods.
    println!("{}", entity.message("Goodbye."));
}

/// Passes a person as a `&dyn Greet`.
fn main() {
    let person = Person {
        name: String::from("Alice"),
    };

    // We can directly pass a reference to `person`,
    // because it implements `Greet`.
    say_hello_and_goodbye(&person);

    // We can also explicitly create a trait object reference:
    let greet_obj: &dyn Greet = &person;
    say_hello_and_goodbye(greet_obj);
}

// ANCHOR_END: example

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
