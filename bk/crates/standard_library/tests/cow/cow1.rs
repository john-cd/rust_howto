// ANCHOR: example

/// Demonstrates the use of `Cow` (Clone-on-Write).
fn main() {
    use std::borrow::Borrow;
    let mut my_string = String::new();

    // Create a Cow that borrows the string literal "Example".
    let example = std::borrow::Cow::from("Example");

    // Borrow the Cow and append it to my_string.
    my_string.push_str(example.borrow());

    println!("{}", my_string);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
