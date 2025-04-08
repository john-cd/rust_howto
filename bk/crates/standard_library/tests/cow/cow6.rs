// ANCHOR: example

fn main() {
    // Create a Cow<'static, str> from a string literal.
    let example = std::borrow::Cow::from("example");

    // Convert the Cow to an owned String.
    println!("{}", example.into_owned());
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
