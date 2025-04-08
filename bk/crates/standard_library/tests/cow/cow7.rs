// ANCHOR: example
fn main() {
    // Create a Cow from a string literal.
    // In this case, it will be a Borrowed Cow.
    let example = std::borrow::Cow::from("example");

    // Convert the Cow to a borrowed reference using `as_ref()`.
    // Then, clone the borrowed reference to create an owned `String`.
    println!("{}", example.as_ref().to_owned());
}
// ANCHOR_END: example
#[test]
fn test() {
    main();
}
