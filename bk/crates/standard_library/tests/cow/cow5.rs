// ANCHOR: example

fn main() {
    // Create a Cow from a string literal.
    let example = std::borrow::Cow::from("example");

    // Convert the Cow to an owned String.
    let s = example.to_string();

    println!("{}", s);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
