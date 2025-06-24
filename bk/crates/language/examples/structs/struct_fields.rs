#![allow(dead_code)]
// ANCHOR: example
#[derive(Debug)]
enum Status {
    Active,
    Inactive,
}

#[derive(Debug)]
struct InnerStruct(i32);

#[derive(Debug)]
struct ComplexStruct<'a> {
    primitive: u32,            // Primitive type.
    tuple_field: (i32, f64),   // Tuple type.
    array_field: [u8; 3],      // Array type.
    enum_field: Status,        // Enum type.
    struct_field: InnerStruct, // Another struct, here a tuple struct.
    reference_field: &'a str,  // Reference type.
}

fn main() {
    let complex = ComplexStruct {
        primitive: 10,
        tuple_field: (5, 3.1),
        array_field: [1, 2, 3],
        enum_field: Status::Active,
        struct_field: InnerStruct(42),
        reference_field: "Hello, world!",
    };

    println!("{:?}", complex);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
