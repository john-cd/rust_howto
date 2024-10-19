#[test]
fn test() {
    let example = std::borrow::Cow::from("example");
    println!("{}", example.as_ref().to_owned());
}
