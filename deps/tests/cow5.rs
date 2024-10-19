#[test]
fn test() {
    let example = std::borrow::Cow::from("example");
    println!("{}", example.to_string());
}
