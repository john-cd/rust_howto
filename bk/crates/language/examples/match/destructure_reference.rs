#![allow(dead_code)]
// ANCHOR: example
fn main() {
    // Given a reference...
    let reference: &i32 = &42;

    // ...we can destructure it into its pointed-to type.
    // Note that `&` is on the left side.
    let &value = reference;

    // `value` is `i32`:
    println!("Type of `value`: {}", std::any::type_name_of_val(&value));
    assert_eq!(value, 42);

    // This is equivalent to dereferencing (following the pointer):
    let _value: i32 = *reference;

    // Note that it won't work if the underlying type is not `Copy`:
    struct NotCopyStruct;
    let _reference = &NotCopyStruct;
    // let &value = _reference; // Error[E0507]: cannot move out of `*reference`
    // which is behind a shared reference.
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
