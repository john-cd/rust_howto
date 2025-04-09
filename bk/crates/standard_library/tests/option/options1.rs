// ANCHOR: example
fn main() {
    // `Some` is a variant of the `Option` enum that represents a value.
    let _some_number = Some(5);

    // `None` is another variant of the `Option` enum that represents the
    // absence of a value.
    let absent_number: Option<i32> = None;
    println!("{:?}", absent_number);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
