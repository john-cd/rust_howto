// ANCHOR: example
fn main() {
    let example = std::borrow::Cow::from("example");
    let s = example.to_string();
    println!("{}", s);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
