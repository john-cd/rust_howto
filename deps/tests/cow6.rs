#[test]
fn test() {
  let example = std::borrow::Cow::from("example");
  println!("{}", example.into_owned());
}
