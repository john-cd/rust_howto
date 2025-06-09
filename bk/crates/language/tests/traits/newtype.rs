// ANCHOR: example
use std::fmt;

// // ERROR: only traits defined in the current crate can be implemented for
// // types defined outside of the crate:
// impl fmt::Display for Vec<String> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "[{}]", self.join(", "))
//     }
// }

/// Tuple struct wrapping the type we want to add a non-local trait to.
///
/// This is the 'newtype pattern'.
struct Wrapper(Vec<String>);

/// Implement a (non-local) trait on the (local) `Wrapper` type:
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
// If you want the new type to have every method the inner type has,
// implement the `Deref` trait.

fn main() {
    println!(
        "{}",
        Wrapper(vec!["example".to_string(), "example 2".to_string()])
    );
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
