fn main() {
    let example = std::borrow::Cow::from("example");
    println!("{}", example.as_ref().to_owned());
}

#[test]
fn test() {
    main();
}
