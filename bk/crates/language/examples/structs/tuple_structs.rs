// TODO Reviewed ??-??-2026.

#![cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "the snippet declares items to illustrate them, not to call them"
    )
)]
// ANCHOR: example
//! Tuple structs and unit-like structs.

/// A tuple struct.
///
/// Note the ( ) and the lack of field names.
/// It behaves like a named tuple.
#[derive(Debug)]
struct Color(pub i32, pub i32, pub i32);
// We choose here to make all fields public.

/// A unit-like struct.
///
/// Unit-like structs are useful when you need to implement a trait on
/// something, but don't have any data that you want to store in the type
/// itself.
#[derive(Debug)]
struct UnitLike; // Note that there are no fields.

/// Creates and accesses tuple structs.
fn main() {
    // Initialize the tuple struct:
    let black = Color(0, 0, 0);
    // Access the inner field with .0, .1, etc.:
    let _red_value = black.0;
    println!("{black:?}");

    let s = UnitLike;
    println!("{s:?}");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the tuple structs example through its entry point.
    #[test]
    fn test_tuple_structs_example_entry_point() {
        main();
    }

    /// Tuple fields are accessed by index; unit-like structs print their name.
    #[test]
    fn test_tuple_structs() {
        let c = Color(1, 2, 3);
        assert_eq!((c.0, c.1, c.2), (1, 2, 3));
        assert_eq!(format!("{c:?}"), "Color(1, 2, 3)");
        assert_eq!(format!("{UnitLike:?}"), "UnitLike");
    }
}
