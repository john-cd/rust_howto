// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates how to use the `use` keyword to bring items into
//! scope without having to write their full path.

/// Modules are used to organize code into logical units.
mod a {
    /// Nested module.
    /// We prefix `mod b` with `pub` to make it public.
    pub mod b {
        /// Public function.
        pub fn fn_in_b() -> &'static str {
            "in b!"
        }

        /// A struct in the nested module.
        pub struct StructInB;
    }

    /// A constant in the module.
    pub const CONST_IN_A: u8 = 1;

    /// A point, as a tuple of coordinates.
    pub type Point = (u8, u8);
}

// Bring the `b` module in scope with the `use` keyword.
use a::b;

// A relative path starts from the current module and uses
// `self`, or an identifier in the current module.
// We could also write: `use self::a::b;`.

/// Refers to items with full and shortened paths and with `use` declarations.
fn main() {
    // Call the function using its full path.
    println!("{}", a::b::fn_in_b());

    // Call the function using its shortened path.
    println!("{}", b::fn_in_b());

    // Use declarations can exist inside of functions.
    // Bring the `StructInB` struct in scope, then refer to it via its short
    // name:
    use a::b::StructInB;
    let _ = StructInB;

    // Use declarations work for constants, type aliases, enums, statics,
    // traits, etc.
    use a::CONST_IN_A;
    let _ = CONST_IN_A;

    use a::Point;
    let _p: Point = (1, 2);
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

    #[test]
    fn test_full_and_shortened_paths_agree() {
        assert_eq!(a::b::fn_in_b(), "in b!");
        assert_eq!(b::fn_in_b(), a::b::fn_in_b());
    }

    #[test]
    fn test_use_for_constants_and_type_aliases() {
        use a::CONST_IN_A;
        use a::Point;
        let p: Point = (1, 2);
        assert_eq!(CONST_IN_A, 1);
        assert_eq!(p.0 + p.1, 3);
    }
}
