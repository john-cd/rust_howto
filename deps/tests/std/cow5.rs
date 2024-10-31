fn main() {
    let example = std::borrow::Cow::from("example");
    let s = example.to_string();
    println!("{}", s);
}

#[test]
fn test() {
    main();
}
