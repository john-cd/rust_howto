#[test]
fn test() {
    let example = std::borrow::Cow::from("example");
    let s = example.to_string();
    println!("{}", s);
}
