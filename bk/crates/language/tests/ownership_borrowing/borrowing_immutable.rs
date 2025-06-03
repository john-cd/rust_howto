// ANCHOR: example
//! Immutable Reference Example.

fn main() {
    let a = 42;

    // Use the `&` operator to create a reference (i.e. "borrow"):
    let b: &i32 = &a;

    // Use `*` to dereference (follow the pointer):
    let c: i32 = *b;
    println!("c: {c}");

    // Shared references are read-only:
    // *b += 1; // ERROR: cannot assign to `*b`, which is behind a `&`
    // reference.

    // The `.` operator used to retrieve a field from a struct or enum also
    // automatically dereferences:
    struct S {
        field: u32,
    }

    let s: &S = &S { field: 3 };
    let _field: u32 = s.field;
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
