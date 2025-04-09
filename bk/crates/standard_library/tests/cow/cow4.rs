// ANCHOR: example
fn main() {
    let mut my_string = String::new();
    // Create a Cow that borrows the string literal "example".
    let example = std::borrow::Cow::from("example");
    // Append the borrowed string to my_string.
    my_string.push_str(&example);
    println!("{}", my_string);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
