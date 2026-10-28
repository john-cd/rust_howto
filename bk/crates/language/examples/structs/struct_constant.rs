// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! An associated constant on a struct.

/// A unit struct.
struct MyToken;

impl MyToken {
    // Inherent implementations can contain associated constants,
    // in addition to associated functions or methods.
    /// An identifier associated with the type.
    const ID: i32 = 1;
}

/// Prints the associated constant.
fn main() {
    println!("ID: {}", MyToken::ID);
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the struct constant example through its entry point.
    #[test]
    fn test_struct_constant_example_entry_point() {
        main();
    }
}
