// ANCHOR: example
fn main() {
    let example = std::borrow::Cow::from("example");
    println!("{}", example.as_ref().to_owned());
}

// ANCHOR_END: example
#[test]
fn test() {
    main();
}
