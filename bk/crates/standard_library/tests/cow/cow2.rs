// ANCHOR: example
fn main() {
    let mut my_string = String::new();
    // Create a `Cow` that borrows the string literal "Example".
    let example = std::borrow::Cow::from("Example");
    // Append the borrowed string to `my_string`.
    my_string.push_str(example.as_ref());
    println!("{}", my_string);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
