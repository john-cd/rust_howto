fn main() {
    let example = std::borrow::Cow::from("example");
    println!("{}", example.into_owned());
}

#[test]
fn test() {
    main();
}
