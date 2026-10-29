// TODO Reviewed ??-??-2026.

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

    // Access the fields individually:
    println!("primitive: {}", complex.primitive);
    println!("tuple_field: {:?}", complex.tuple_field);
    println!("array_field: {:?}", complex.array_field);
    println!("struct_field: {}", complex.struct_field.0);
    println!("reference_field: {}", complex.reference_field);
    match complex.enum_field {
        Status::Active => println!("The item is active."),
        Status::Inactive => println!("The item is inactive."),
    }
    println!("{:?}", Status::Inactive);
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the struct fields example through its entry point.
    #[test]
    fn test_struct_fields_example_entry_point() {
        main();
    }

    /// The derived `Debug` output lists every field.
    #[test]
    fn test_complex_struct_debug() {
        let complex = ComplexStruct {
            primitive: 10,
            tuple_field: (5, 3.5),
            array_field: [1, 2, 3],
            enum_field: Status::Inactive,
            struct_field: InnerStruct(42),
            reference_field: "hi",
        };
        assert_eq!(
            format!("{complex:?}"),
            "ComplexStruct { primitive: 10, tuple_field: (5, 3.5), array_field: [1, 2, 3], enum_field: Inactive, struct_field: InnerStruct(42), reference_field: \"hi\" }"
        );
    }
}
