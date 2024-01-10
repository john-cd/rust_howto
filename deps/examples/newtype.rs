use std::fmt;

struct Wrapper(Vec<String>); // tuple struct wrapping the type we want to add a non-local trait to.

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
// If we wanted the new type to have every method the inner type has, implement
// the `Deref` trait.

fn main() {
    println!(
        "{}",
        Wrapper(vec!["example".to_string(), "example 2".to_string()])
    );
}
