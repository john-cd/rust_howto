#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! A struct with fields of many types.

/// A status.
#[derive(Debug)]
enum Status {
    /// The item is active.
    Active,
    /// The item is inactive.
    Inactive,
}

/// A tuple struct.
#[derive(Debug)]
struct InnerStruct(i32);

/// A struct with a field of each kind of type.
#[derive(Debug)]
struct ComplexStruct<'a> {
    /// A primitive.
    primitive: u32, // Primitive type.
    /// A tuple.
    tuple_field: (i32, f64), // Tuple type.
    /// An array.
    array_field: [u8; 3], // Array type.
    /// An enum.
    enum_field: Status, // Enum type.
    /// Another struct.
    struct_field: InnerStruct, // Another struct, here a tuple struct.
    /// A reference.
    reference_field: &'a str, // Reference type.
}

/// Creates and prints the struct.
fn main() {
    let complex = ComplexStruct {
        primitive: 10,
        tuple_field: (5, 3.1),
        array_field: [1, 2, 3],
        enum_field: Status::Active,
        struct_field: InnerStruct(42),
        reference_field: "Hello, world!",
    };

    println!("{complex:?}");
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
