#![allow(dead_code)]
#![allow(unused_variables)]

// TODO Reviewed ??-??-2026
// ANCHOR: example
//! A lifetime parameter on a struct and on its `impl` blocks.

/// A struct that holds a reference to a string slice.
struct Excerpt<'a> {
    /// The excerpt, borrowed from a longer text.
    part: &'a str,
}

/// An implementation only defined for the `'static` lifetime.
impl Excerpt<'static> {
    /// A method that exists only when the lifetime is `'static`.
    fn only_if_static(&self) {}
}

/// An implementation block for the struct.
/// The `'_` notation indicates that the struct takes a lifetime parameter,
/// but it does not matter which, because the lifetime is not referenced in the
/// block.
impl Excerpt<'_> {
    /// Returns a constant level, whatever the lifetime.
    fn level(&self) -> i32 {
        3
    }
}

/// A generic implementation block.
/// Note the lifetime parameter `'a` after `impl`.
impl<'a> Excerpt<'a> {
    /// Returns the excerpt with its original lifetime.
    fn part(&self) -> &'a str {
        self.part
    }
}

/// Creates an excerpt and calls its methods.
fn main() {
    // Create an instance of the `struct` using a string literal of type
    // `'static`. Therefore 'a = 'static.
    let e: Excerpt<'static> = Excerpt { part: "a part" };
    // `level` and `part` are defined for any lifetime.
    println!("{} level {}", e.part(), e.level());
    // `only_if_static` is only defined for `'static`.
    e.only_if_static();

    // Let's create another instance with a lifetime shorter than `'static`:
    let another_part = String::from("another part");
    let e2: Excerpt<'_> = Excerpt {
        part: &another_part,
    };
    // e2.only_if_static(); // ERROR: argument requires that `another_part` is
    // borrowed for `'static`
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
