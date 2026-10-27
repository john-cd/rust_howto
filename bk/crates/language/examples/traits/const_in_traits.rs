#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Associated constants in traits.

/// A trait demonstrating the use of associated constants.
trait Example {
    /// An associated constant without a default value.
    ///
    /// Implementors of this trait must provide a value for this constant.
    const CONST_NO_DEFAULT: i32;

    /// An associated constant with a default value.
    ///
    /// Implementors of this trait can optionally override this default.
    const CONST_WITH_DEFAULT: i32 = 99;
}

/// A struct that implements the `Example` trait.
struct S;

impl Example for S {
    const CONST_NO_DEFAULT: i32 = 0;
}

/// Prints the associated constants.
fn main() {
    println!("{} {}", S::CONST_NO_DEFAULT, S::CONST_WITH_DEFAULT);
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the const in traits example through its entry point.
    #[test]
    fn test_const_in_traits_example_entry_point() {
        main();
    }
}
