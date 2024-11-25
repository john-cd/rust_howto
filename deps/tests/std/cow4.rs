// ANCHOR: example
fn main() {
    let mut my_string = String::new();
    let example = std::borrow::Cow::from("example");
    my_string.push_str(&example);
    println!("{}", my_string);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
