// ANCHOR: example
fn main() {
    let example = std::borrow::Cow::from("example");
    println!("{}", example.into_owned());
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
